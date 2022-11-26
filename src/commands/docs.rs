use super::prelude::*;
use crate::content::docs;
use itertools::Itertools;
use serenity::{
    json::{json, Value},
    model::application::{
        command::CommandOptionType,
        interaction::{application_command::CommandDataOptionValue, MessageFlags},
    },
};
use sublime_fuzzy::{FuzzySearch, Scoring};

pub(crate) struct DocsSlashCommand;

impl SlashCommandRegister for DocsSlashCommand {
    const NAME: &'static str = "docs";

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(Self::NAME)
            .description("Searches through GB Studio online documentation")
            .create_option(|option| {
                option
                    .name("query")
                    .description("Documentation page to search for")
                    .kind(CommandOptionType::String)
                    .set_autocomplete(true)
            })
    }
}

#[async_trait]
impl SlashCommandRespond for DocsSlashCommand {
    async fn respond(
        ctx: Context,
        interaction: &ApplicationCommandInteraction,
    ) -> serenity::Result<()> {
        let options = &interaction.data.options;

        let mut content = docs::ROOT; // Default response content.
        for option in options {
            let resolved_option = option
                .resolved
                .as_ref()
                .expect("Failed to resolve a slash command option");

            match option.name.as_str() {
                "query" => {
                    let resolved_option_value = match resolved_option {
                        CommandDataOptionValue::String(s) => s,
                        invalid_value => {
                            unreachable!("Invalid object resolution {invalid_value:?} for option query of commands docs");
                        }
                    };

                    content = docs::INDEX
                        .entries()
                        .max_by_key(|entry| {
                            FuzzySearch::new(resolved_option_value, entry.0)
                                .case_insensitive()
                                .best_match()
                        })
                        .map(|entry| entry.1)
                        .unwrap();
                }
                option_name => {
                    unreachable!("Unhandled option {option_name} of command docs");
                }
            }
        }

        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| {
                        data.content(content).flags(MessageFlags::SUPPRESS_EMBEDS)
                    })
            })
            .await
    }
}

#[async_trait]
impl SlashCommandAutocomplete for DocsSlashCommand {
    async fn autocomplete(
        ctx: Context,
        interaction: &AutocompleteInteraction,
    ) -> serenity::Result<()> {
        let options = &interaction.data.options;

        if let Some(focused_option) = options.iter().find(|option| option.focused) {
            let resolved_option = focused_option
                .resolved
                .as_ref()
                .expect("Failed to resolve a slash command option");

            match focused_option.name.as_str() {
                "query" => {
                    let resolved_option_value = match resolved_option {
                        CommandDataOptionValue::String(s) => s,
                        invalid_value => {
                            unreachable!("Invalid object resolution {invalid_value:?} for option query of commands docs");
                        }
                    };

                    let scoring = Scoring::emphasize_word_starts();
                    let choices: Vec<_> = docs::INDEX
                        .keys()
                        .sorted_by_key(|key| {
                            FuzzySearch::new(resolved_option_value, key)
                                .score_with(&scoring)
                                .case_insensitive()
                                .best_match()
                        })
                        .rev()
                        .take(5)
                        .map(|s| {
                            json!({
                                "name": s,
                                "value": s
                            })
                        })
                        .collect();

                    return interaction
                        .create_autocomplete_response(&ctx.http, |autocomplete| {
                            autocomplete.set_choices(Value::Array(choices))
                        })
                        .await;
                }
                option_name => {
                    unreachable!(
                        "Unhandled autocomplete request for option {option_name} of command docs"
                    );
                }
            }
        }

        Ok(())
    }
}
