use std::fmt::format;
use serenity::all::{ButtonStyle, ChannelId, ComponentInteraction, CreateEmbedAuthor, CreateEmbedFooter, GuildId, Member, MessageId, ModalInteraction};
use serenity::builder::{CreateButton, CreateEmbed, CreateMessage};
use serenity::client;
use serenity::model::Timestamp;
use crate::emojis::{emoji_counterclockwise_arrows, emoji_party_popper, emoji_warning};
use crate::secrets;

pub async fn handle_member_join(ctx: &client::Context, new_member: &Member) -> Result<(), serenity::Error> {
    let secrets = secrets::Secrets::get_secrets();
    let guild_id = GuildId::new(secrets.guild_id);
    let channel_id = ctx.http.get_channel(ChannelId::new(secrets.welcome_channel_id)).await?;

    let initial_message = CreateMessage::new().content("Loading...");
    let message = channel_id.id().send_message(&ctx.http, initial_message).await?;

    let joined_message = create_joined_message(message.id, new_member.clone());

    Ok(())
}

pub fn create_joined_message(message_id: MessageId, new_member: Member) -> CreateMessage {
    let info_author = CreateEmbedAuthor::new("New Member Joined");

    let timestamp: Timestamp = Timestamp::now();

    let footer = CreateEmbedFooter::new(format!("Member ID: {}", interaction.user.id.to_string()));

    CreateMessage::new()
}

pub fn create_new_member_message(interaction: &ModalInteraction, discord_user_id: u64, character_name: String, realm: String) -> CreateMessage {
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
        .field("Status", format!("{} Awaiting Onboarding", emoji_counterclockwise_arrows()), true)
        .footer(footer)
        .timestamp(timestamp);

    let button_complete_registration = CreateButton::new("button_complete_registration")
        .style(ButtonStyle::Success)
        .label("Mark Complete");

    let button_change_chapter = CreateButton::new("button_change_chapter")
        .style(ButtonStyle::Secondary)
        .label("Change Chapter");

    let message = CreateMessage::new()
        .embed(info_embed)
        .button(button_complete_registration)
        .button(button_change_chapter);

    message
}

pub fn create_completed_onboarding_message(interaction: ComponentInteraction) -> CreateMessage {
    let info_author = CreateEmbedAuthor::new("Member Onboarding Submitted");

    let timestamp: Timestamp = Timestamp::now();

    let footer = CreateEmbedFooter::new(format!("Member ID: {}", interaction.user.id.to_string()));

    let member_mention = interaction.message.embeds[0].fields[0].value.clone();
    let character_name = interaction.message.embeds[0].fields[1].value.clone();
    let realm = interaction.message.embeds[0].fields[2].value.clone();

    let info_embed = CreateEmbed::new()
        .author(info_author)
        .title(format!("{} IMPORTANT REMINDER", emoji_warning()))
        .description("Warning! Only mark complete after promoting this member in-game to full member status.")
        .field("Member", member_mention, true)
        .field("Character Name", character_name, true)
        .field("Realm", realm, true)
        .field("Status", format!("{} Onboarding Complete!", emoji_party_popper()), true)
        .field("Message ID", interaction.message.id.to_string(), true)
        .footer(footer)
        .timestamp(timestamp);

    let button_undo_completed = CreateButton::new("button_undo_completed");

    let message = CreateMessage::new()
        .embed(info_embed)
        .button(button_undo_completed);

    message
}
