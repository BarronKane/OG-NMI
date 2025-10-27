use std::ffi::CString;
use serenity::all::{ButtonStyle, ChannelId, CommandInteraction, ComponentInteraction, CreateEmbedAuthor, CreateMessage, InputTextStyle, ModalInteraction, Role};
use serenity::all::Unresolved::RoleId;
use serenity::builder::{CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal, CreateInteractionResponseMessage, CreateEmbed, CreateButton};
use serenity::client::Context;
use serenity::futures::{StreamExt, pin_mut};
use crate::chapters::Chapters;
use crate::emojis::{emoji_party_popper, emoji_warning};
use crate::member_info::{create_new_member_buttons, create_new_member_embeds, push_member_completion_message};
use crate::secrets;

pub async fn nmi_modal(ctx: &Context, interaction: &ComponentInteraction) -> Result<(), serenity::Error> {
    let chapter_value = CreateInputText::new(
        InputTextStyle::Short,
        "Chapter Number:",
        "chapter_number"
    ).required(true).min_length(1).max_length(2).placeholder("0 for Aegwynn, etc...");

    let character_name = CreateInputText::new(
        InputTextStyle::Short,
        "Character Name:",
        "character_name"
    ).required(true).min_length(2).max_length(14).placeholder("Bjork");

    let realm_name = CreateInputText::new(
        InputTextStyle::Short,
        "Realm Name:",
        "realm_name"
    ).required(true).min_length(2).max_length(20).placeholder("Tichondrius");

    let modal = CreateInteractionResponse::Modal(
        CreateModal::new("nmi_modal", "NMI Character Registration")
            .components(vec![
                CreateActionRow::InputText(chapter_value),
                CreateActionRow::InputText(character_name),
                CreateActionRow::InputText(realm_name)
            ])
    );

    interaction.create_response(ctx.http.clone(), modal).await?;

    Ok(())
}

pub async fn nmi_modal_response(ctx: &Context, interaction: &ModalInteraction) -> Result<(), serenity::Error> {
    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Please standby...").ephemeral(true))).await?;
    let default_string = String::new();
    let chapter_number = &interaction
        .data
        .components
        .get(0)
        .and_then(|row| row.components.get(0))
        .and_then(|component| {
            if let serenity::all::ActionRowComponent::InputText(input) = component {
                input.value.as_ref()
            } else {
                None
            }
        })
        .unwrap_or(&default_string);

    let _chapter = crate::chapters::Chapters::load();
    let chapter_number = chapter_number.parse::<u64>().unwrap_or(99);

    if (chapter_number == 99) || (chapter_number > (_chapter.get_count() as u64 - 1)) {
        interaction.create_response(
            ctx.http.clone(),
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default()
                    .content("Invalid chapter number. Please try again.")
                    .ephemeral(true)
            ),
        ).await?;
        return Ok(());
    }

    let default_character_name = String::new();
    let character_name =
        &interaction
            .data
            .components
            .get(1)
            .and_then(|row| row.components.get(0))
            .and_then(|component| {
                if let serenity::all::ActionRowComponent::InputText(input) = component {
                    input.value.as_ref()
                } else {
                    None
                }
            })
            .unwrap_or(&default_character_name);

    let default_realm_name = String::new();
    let realm_name = &interaction
        .data
        .components
        .get(2)
        .and_then(|row| row.components.get(0))
        .and_then(|component| {
            if let serenity::all::ActionRowComponent::InputText(input) = component {
                input.value.as_ref()
            } else {
                None
            }
        })
        .unwrap_or(&default_realm_name);
    
    let secrets = secrets::Secrets::get_secrets();

    let guild_id = interaction.guild_id.ok_or(serenity::Error::Other("No guild ID found"))?;
    let member = guild_id.member(&ctx.http, interaction.user.id).await?;

    let new_member_role_id = serenity::model::id::RoleId::new(secrets.new_member_role_id);
    //let new_member_role = guild_id.role(&ctx.http, new_member_role_id).await?;

    let member_role_id = serenity::model::id::RoleId::new(secrets.member_role_id);
    //let member_role = guild_id.role(&ctx.http, member_role_id).await?;

    let chapter_role_id = serenity::model::id::RoleId::new(_chapter.get_by_id(chapter_number as usize).ok_or(serenity::Error::Other("No chapter found"))?.role_id.clone());
    //let chapter_role = guild_id.role(&ctx.http, chapter_role_id).await?;

    member.remove_role(&ctx.http, new_member_role_id).await?;
    member.add_role(&ctx.http, member_role_id).await?;

    member.add_role(&ctx.http, chapter_role_id).await?;

    interaction.user.direct_message(&ctx.http, CreateMessage::new()
        // TODO: Embed
        .content(
            format!("Congratulations! {} Welcome to the Old Gods! Your GM will review your character and promote them in-game.", emoji_party_popper()))
    ).await?;
    
    let channel_id = secrets.nmi_channel_id;
    let channel = ctx.http.get_channel(ChannelId::new(channel_id)).await?;

    let new_msg_embeds = create_new_member_embeds(interaction, member.user.id.to_string().parse::<u64>().expect("Couldn't parse UserId as u64."), character_name.to_string(), realm_name.to_string());
    let new_msg_buttons = create_new_member_buttons();

    push_member_completion_message(ctx, &member, channel, new_msg_embeds, new_msg_buttons).await?;

    Ok(())
}
