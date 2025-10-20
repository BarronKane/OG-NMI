use serenity::all::{CommandInteraction, InputTextStyle};
use serenity::builder::{CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal};
use serenity::client::Context;

pub async fn nmi_modal(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let chapter_value = CreateInputText::new(
        InputTextStyle::Short,
        "Chapter Number:",
        "chapter_number"
    ).required(true).min_length(1).max_length(2).placeholder("1 for Aegwynn, etc...");

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

    interaction.create_response(ctx.http.clone(), modal).await
}