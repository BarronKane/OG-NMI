use std::sync::RwLock;
use turso::{Builder, Connection, Error, Rows};

static SQLITE_CONN: RwLock<Option<Connection>> = RwLock::new(None);

#[derive(Debug)]
pub enum MemberJoinMessageStage {
    NewMember = 0,
    Onboarding = 1,
    Completed = 2,
}

impl From<i64> for MemberJoinMessageStage {
    fn from(value: i64) -> Self {
        match value {
            0 => MemberJoinMessageStage::NewMember,
            1 => MemberJoinMessageStage::Onboarding,
            2 => MemberJoinMessageStage::Completed,
            _ => MemberJoinMessageStage::NewMember,
        }
    }
}

impl ToString for MemberJoinMessageStage {
    fn to_string(&self) -> String {
        match self {
            MemberJoinMessageStage::NewMember => "0".to_string(),
            MemberJoinMessageStage::Onboarding => "1".to_string(),
            MemberJoinMessageStage::Completed => "2".to_string(),
            _ => "0".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct MemberJoinMessage {
    pub id: i64,
    pub discord_user_id: u64,
    pub message_id: u64,
    pub stage: MemberJoinMessageStage,
}

impl MemberJoinMessage {
    async fn get_connection() -> Result<Connection, Error> {
        if let Ok(cache) = SQLITE_CONN.read() {
            if let Some(db) = cache.as_ref() {
                return Ok(db.clone());
            }
        }

        let db = Builder::new_local("sqlite.db").build().await?;
        let conn = db.connect()?;
        if let Ok(mut cache) = SQLITE_CONN.write() {
            *cache = Some(conn.clone());
        }

        // u64 values need to be stored as TEXT. Internally, INTEGER is i64.
        conn.execute(
            "CREATE TABLE member_join_messages (\
            id INTEGER PRIMARY KEY,\
            discord_user_id TEXT,\
            message_id TEXT,\
            stage INTEGER)", ()
        ).await?;

        Ok(conn)
    }

    pub async fn push_message(discord_user_id: String, message_id: String, stage: MemberJoinMessageStage) -> Result<(), Error> {
        let conn = Self::get_connection().await?;
        let in_stage = stage as i32;
        conn.execute(
            "INSERT INTO member_join_messages (discord_user_id, message_id, stage) VALUES (?1, ?2, ?3)",
            [discord_user_id, message_id, in_stage.to_string()]
        ).await?;

        Ok(())
    }

    pub async fn update_message(&self, stage: MemberJoinMessageStage) -> Result<(), Error> {
        let conn = Self::get_connection().await?;
        let in_stage = stage as i32;
        conn.execute(
            "UPDATE member_join_messages SET stage = ?1 WHERE id = ?2",
            [in_stage.to_string(), self.id.to_string()]
        ).await?;

        Ok(())
    }

    pub async fn get_message_by_discord_user_id(discord_user_id: String) -> Result<MemberJoinMessage, Error> {
        let conn = Self::get_connection().await?;
        let mut rows = conn.query(
            "SELECT * FROM member_join_messages WHERE discord_user_id = ?1",
            [discord_user_id]
        ).await?;

        let join_message = Self::collect_from_db(&mut rows).await?;

        Ok(join_message)
    }

    pub async fn get_message_by_message_id(message_id: String) -> Result<MemberJoinMessage, Error> {
        let conn = Self::get_connection().await?;
        let mut rows = conn.query(
            "SELECT * FROM member_join_messages WHERE message_id = ?1",
            [message_id]
        ).await?;

        let join_message = Self::collect_from_db(&mut rows).await?;

        Ok(join_message)
    }

    async fn collect_from_db(rows: &mut Rows) -> Result<MemberJoinMessage, Error> {
        let mut join_message = MemberJoinMessage {
            id: 0,
            discord_user_id: 0,
            message_id: 0,
            stage: MemberJoinMessageStage::NewMember,
        };

        while let Some(row) = rows.next().await? {
            join_message.id = *row.get_value(0)?.as_integer().expect("Could not get ID from db.");
            join_message.discord_user_id = row.get_value(1)?.as_text().expect("Could not get Discord User ID from db.").parse::<u64>().expect("Could not parse discord id as u64 from db.");
            join_message.message_id = row.get_value(2)?.as_text().expect("Could not get Message ID from db.").parse::<u64>().expect("Could not parse message id as u64 from db.");
            let out_stage = row.get_value(3)?.as_text().expect("Could not get stage from db.").parse::<i64>().expect("Could not parse stage as i64.");
            join_message.stage = MemberJoinMessageStage::from(out_stage);
        }

        Ok(join_message)
    }
}