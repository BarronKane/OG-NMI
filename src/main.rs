mod nmi_handler;
mod chapters;
mod message_command;
mod secrets;
mod emojis;
mod member_info;
mod member_db;

use serenity::all::{Interaction, Member};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use serde_json;
use serde::{Deserialize, Serialize};
use crate::message_command::send_welcome_message;
use crate::chapters::Chapters;
use crate::member_info::{handle_complete_onboarding, handle_member_join, handle_undo_completion};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("The bot is connected!");

        let secrets = secrets::Secrets::get_secrets();

        let guild_id = GuildId::new(secrets.guild_id);

        let command = message_command::register_welcome_message_command().await;
        guild_id.set_commands(&ctx.http, vec![command]).await.expect("Could not register commands.");
    }
    
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let result = handle_member_join(&ctx, &new_member).await;
        match result {
            Ok(_) => {

            }
            Err(e) => {
                println!("Error handling member join: {}", e);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Component(component) = interaction.clone() {
            if component.data.custom_id == "nmi_button" {
                let response = nmi_handler::nmi_modal(&ctx, &component).await;

                match response {
                    Ok(_) => {

                    }
                    Err(e) => {
                        println!("Error handling NMI button: {}", e);
                    }
                }
            }

            if component.data.custom_id == "guest_button" {
                // TODO
            }

            if component.data.custom_id == "button_complete_registration" {
                let response = handle_complete_onboarding(&ctx, component.clone()).await;
                match response {
                    Ok(_) => {

                    }
                    Err(e) => {
                        println!("Error handling complete onboarding: {}", e);
                    }
                }
            }

            if component.data.custom_id == "button_undo_completed" {
                let result = handle_undo_completion(&ctx, component).await;
                match result {
                    Ok(_) => {

                    }
                    Err(e) => {
                        println!("Error handling undo onboarding: {}", e);
                    }
                }
            }
        }

        if let Interaction::Modal(modal) = interaction.clone() {
            if modal.data.custom_id == "nmi_modal" {
                let response = nmi_handler::nmi_modal_response(&ctx, &modal).await;
                match response {
                    Ok(_) => {

                    }
                    Err(e) => {
                        println!("Error handling NMI modal: {}", e);
                    }
                }
            }
        }

        if let Interaction::Command(command) = interaction.clone() {
            if command.data.name.as_str() == "create_welcome_message" {
                send_welcome_message(ctx, command).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Load chapters from JSON
    let chapters = Chapters::load();
    println!("{}", chapters.to_formatted_list());

    let secrets = secrets::Secrets::get_secrets();

    // Only intents needed for interactions, may be none.
    let intents =
        GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&secrets.token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
