use std::fmt::{format, Debug};
use std::sync::RwLock;
use serenity::all::{ButtonStyle, Channel, ChannelId, ComponentInteraction, CreateEmbedAuthor, CreateEmbedFooter, GuildId, Member, MessageId, ModalInteraction};
use serenity::builder::{CreateButton, CreateEmbed, CreateMessage, EditMessage};
use serenity::client;
use serenity::model::Timestamp;
use turso::{
    Builder,
    Connection,
    Error};
use crate::emojis::{emoji_counterclockwise_arrows, emoji_party_popper, emoji_warning};
use crate::member_db::{MemberJoinMessage, MemberJoinMessageStage};
use crate::secrets;

pub async fn handle_member_join(ctx: &client::Context, new_member: &Member) -> Result<(), serenity::Error> {
    let secrets = secrets::Secrets::get_secrets();
    let channel_id = ctx.http.get_channel(ChannelId::new(secrets.nmi_channel_id)).await?;

    let joined_message = create_joined_message(new_member.clone());
    let message = channel_id.id().send_message(&ctx.http, joined_message).await?;

    let message_result = MemberJoinMessage::push_message(
        new_member.user.id.to_string(),
        message.id.to_string(),
        MemberJoinMessageStage::NewMember
    ).await;

    match message_result {
        Ok(_) => {

        },
        Err(e) => {
            println!("Error pushing message to database: {}", e);
        }
    }

    Ok(())
}

pub async fn push_member_completion_message(ctx: &client::Context, new_member: &Member, channel: Channel, new_embeds: Vec<CreateEmbed>, new_buttons: Vec<CreateButton>) -> Result<(), serenity::Error> {
    let previous_message_result = MemberJoinMessage::get_message_by_discord_user_id(new_member.user.id.to_string()).await;
    let previous_message: MemberJoinMessage;

    match previous_message_result {
        Ok(message) => {
            previous_message = message;
            let mut previous_message_id = ctx.http.get_message(channel.id(), MessageId::new(previous_message.message_id)).await?;
            let mut edited_msg = EditMessage::new().embeds(new_embeds);
            for button in new_buttons {
                edited_msg = edited_msg.button(button);
            }
            previous_message_id.edit(&ctx.http, edited_msg).await?;
            let result = previous_message.update_message(MemberJoinMessageStage::Onboarding).await;
            match result {
                Ok(_) => {

                },
                Err(e) => {
                    println!("Error updating previous message in database: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error getting previous message from database: {}", e);
            let mut new_message = CreateMessage::new().embeds(new_embeds);
            for button in new_buttons {
                new_message = new_message.button(button);
            }
            channel.id().send_message(&ctx.http, new_message).await?;
        }
    }

    Ok(())
}

// TODO: Pass channel in instead.
pub async fn handle_complete_onboarding(ctx: &client::Context, interaction: ComponentInteraction) -> Result<(), serenity::Error> {
    let message_id = interaction.message.id;
    let message_result = MemberJoinMessage::get_message_by_message_id(message_id.to_string()).await;

    match message_result {
        Ok(message) => {
            let mut message_id = ctx.http.get_message(interaction.channel_id, message_id).await?;
            let mut edit_message = EditMessage::new()
                .embeds(create_completed_onboarding_embeds(&interaction));
            let buttons = create_completed_onboarding_buttons();
            for button in buttons {
                edit_message = edit_message.button(button);
            }
            message_id.edit(&ctx.http, edit_message).await?;
            let result = message.update_message(MemberJoinMessageStage::Completed).await;
            match result {
                Ok(_) => {

                },
                Err(e) => {
                    println!("Error updating message in database: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error getting message from database: {}", e);
            let mut new_message = CreateMessage::new().embeds(create_completed_onboarding_embeds(&interaction));
            let buttons = create_completed_onboarding_buttons();
            for button in buttons {
                new_message = new_message.button(button);
            }
            interaction.channel_id.send_message(&ctx.http, new_message).await?;
        }
    }

    Ok(())
}

pub async fn handle_undo_completion(ctx: &client::Context, interaction: ComponentInteraction) -> Result<(), serenity::Error> {
    let message_id = interaction.message.id;
    let message_result = MemberJoinMessage::get_message_by_message_id(message_id.to_string()).await;

    match message_result {
        Ok(message) => {
            let mut message_id = ctx.http.get_message(interaction.channel_id, message_id).await?;
            let mut edit_message = EditMessage::new()
                .embeds(create_undo_onboarding_embeds(&interaction));
            let buttons = create_undo_member_buttons();
            for button in buttons {
                edit_message = edit_message.button(button);
            }
            message_id.edit(&ctx.http, edit_message).await?;
            let result = message.update_message(MemberJoinMessageStage::Onboarding).await;
            match result {
                Ok(_) => {

                }
                Err(e) => {
                    println!("Error updating message in database: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error getting message from database: {}", e);
            let mut new_message = CreateMessage::new().embeds(create_undo_onboarding_embeds(&interaction));
            let buttons = create_undo_member_buttons();
            for button in buttons {
                new_message = new_message.button(button);
            }
            interaction.channel_id.send_message(&ctx.http, new_message).await?;
        }
    }

    Ok(())
}

pub fn create_joined_message(new_member: Member) -> CreateMessage {
    let info_author = CreateEmbedAuthor::new("New Member Joined");

    let timestamp: Timestamp = Timestamp::now();

    let footer = CreateEmbedFooter::new(format!("Member ID: {}", new_member.user.id));

    let info_embed = CreateEmbed::new()
        .author(info_author)
        .field("Member", format!("<@{}>", new_member.user.id), true)
        .field("Character Name", emoji_warning(), true)
        .field("Realm", emoji_warning(), true)
        .field("User Id", new_member.user.id.to_string(), true)
        .field("Status", format!("{} Awaiting Onboarding", emoji_counterclockwise_arrows()), true)
        .timestamp(timestamp);

    let message = CreateMessage::new()
        .embed(info_embed);

    message
}

pub fn create_new_member_embeds(interaction: &ModalInteraction, discord_user_id: u64, character_name: String, realm: String) -> Vec<CreateEmbed> {
    let info_author = CreateEmbedAuthor::new("Member Onboarding Submitted");

    let timestamp: Timestamp = Timestamp::now();

    let footer = CreateEmbedFooter::new(format!("Member ID: {}", interaction.user.id.to_string()));

    let info_embed = CreateEmbed::new()
        .author(info_author)
        .title(format!("{} IMPORTANT REMINDER", emoji_warning()))
        .description("Warning! Only mark complete after promoting this member in-game to full member status.")
        .field("Member", format!("<@{}>", interaction.user.id), true)
        .field("Character Name", character_name, true)
        .field("Realm", realm, true)
        .field("User Id", discord_user_id.to_string(), true)
        .field("Status", format!("{} Awaiting Officer Approval", emoji_counterclockwise_arrows()), true)
        //.footer(footer)
        .timestamp(timestamp);

    vec![info_embed]
}

pub fn create_new_member_buttons() -> Vec<CreateButton> {
    let button_complete_registration = CreateButton::new("button_complete_registration")
        .style(ButtonStyle::Success)
        .label("Mark Complete");

    let button_change_chapter = CreateButton::new("button_change_chapter")
        .style(ButtonStyle::Secondary)
        .label("Change Chapter");

    vec![button_complete_registration, button_change_chapter]
}

pub fn create_completed_onboarding_embeds(interaction: &ComponentInteraction) -> Vec<CreateEmbed> {
    let info_author = CreateEmbedAuthor::new("Member Onboarding Submitted");

    let timestamp: Timestamp = Timestamp::now();

    let footer = CreateEmbedFooter::new(format!("Member ID: {}", interaction.user.id.to_string()));

    let message = interaction.message.clone();

    let member_mention = message.embeds[0].fields[0].value.clone();
    let character_name = message.embeds[0].fields[1].value.clone();
    let realm = message.embeds[0].fields[2].value.clone();
    let user_id = message.embeds[0].fields[3].value.clone();

    let info_embed = CreateEmbed::new()
        .author(info_author)
        .title(format!("{} IMPORTANT REMINDER", emoji_warning()))
        .description("Warning! Only mark complete after promoting this member in-game to full member status.")
        .field("Member", member_mention, true)
        .field("Character Name", character_name, true)
        .field("Realm", realm, true)
        .field("User Id", user_id, true)
        .field("Status", format!("{} Onboarding Complete!", emoji_party_popper()), true)
        //.footer(footer)
        .timestamp(timestamp);

    vec![info_embed]
}

pub fn create_completed_onboarding_buttons() -> Vec<CreateButton> {
    let button_undo_completed = CreateButton::new("button_undo_completed")
        .style(ButtonStyle::Danger)
        .label("Undo");

    vec![button_undo_completed]
}

pub fn create_undo_onboarding_embeds(interaction: &ComponentInteraction) -> Vec<CreateEmbed> {
    let info_author = CreateEmbedAuthor::new("Member Onboarding Submitted");

    let timestamp: Timestamp = Timestamp::now();

    let footer = CreateEmbedFooter::new(format!("Member ID: {}", interaction.user.id.to_string()));

    let message = interaction.message.clone();

    let member_mention = message.embeds[0].fields[0].value.clone();
    let character_name = message.embeds[0].fields[1].value.clone();
    let realm = message.embeds[0].fields[2].value.clone();
    let user_id = message.embeds[0].fields[3].value.clone();

    let info_embed = CreateEmbed::new()
        .author(info_author)
        .title(format!("{} IMPORTANT REMINDER", emoji_warning()))
        .description("Warning! Only mark complete after promoting this member in-game to full member status.")
        .field("Member", member_mention, true)
        .field("Character Name", character_name, true)
        .field("Realm", realm, true)
        .field("User Id", user_id, true)
        .field("Status", format!("{} Onboarding Complete!", emoji_party_popper()), true)
        //.footer(footer)
        .timestamp(timestamp);

    vec![info_embed]
}

pub fn create_undo_member_buttons() -> Vec<CreateButton> {
    let button_complete_registration = CreateButton::new("button_complete_registration")
        .style(ButtonStyle::Success)
        .label("Mark Complete");

    let button_change_chapter = CreateButton::new("button_change_chapter")
        .style(ButtonStyle::Secondary)
        .label("Change Chapter");

    vec![button_complete_registration, button_change_chapter]
}
