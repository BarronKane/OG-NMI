use serenity::all::{ButtonStyle, CommandInteraction, CreateCommand, CreateInteractionResponseMessage, CreateMessage, Member, MessageBuilder, Permissions, ResolvedOption};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::builder;
use serenity::builder::CreateInteractionResponse;
use serenity::model::colour;
use serenity::prelude::*;
use crate::secrets;
use crate::chapters::Chapters;

pub async fn send_welcome_message(ctx: Context, command: CommandInteraction) {
    let member = command.clone().member.expect("Could not get member.");
    let secrets = secrets::Secrets::get_secrets();
    if secrets.authorized_ids.contains(&member.user.id.get()) == false {
        let message = command.clone().create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default()
                    .ephemeral(true)
                    .content("You are not authorized to use this command.")
            ))
            .await
            .expect("Could not send denial response.");
        return;
    }

    command.create_response(
        &ctx,
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).content(
            "Welcome message sending in progress..",
        ))
    ).await.expect("Could not acknowledge command");

    let message = create_chapter_message();
    let result = command.channel_id.send_message(ctx.http, message).await;
    match result {
        Ok(_) => println!("Sent welcome message to {}", member.user.name),
        Err(why) => eprintln!("Error sending welcome message: {:?}", why),
    }
}

pub async fn register_welcome_message_command() -> CreateCommand {
    CreateCommand::new("create_welcome_message").description("Ads the NMI Welcome Message.")
}

fn create_chapter_message() -> CreateMessage {
    let chapters = Chapters::load();
    let chapter_list = chapters.to_formatted_list();

    let mut body = chapter_list;
    body += "\n\n";
    body +="Welcome to the Old Gods! Please find your chapter number above and fill in the form below!";

    let nmi_button = builder::CreateButton::new("nmi_button")
        .label("Chapter Form.")
        .style(ButtonStyle::Primary);

    let guest_button = builder::CreateButton::new("guest_button")
        .label("I'm a Guest!")
        .style(ButtonStyle::Secondary);

    let embed = builder::CreateEmbed::default()
        .color(colour::Color::from_rgb(167, 36, 255))
        .title("New Member Info")
        .description(body);

    let message = builder::CreateMessage::new()
        .embed(embed)
        .button(nmi_button);
        //button(guest_button);

    message
}