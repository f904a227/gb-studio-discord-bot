use super::prelude::*;
use crate::content::docs;
use itertools::Itertools;
use serenity::model::application::{
    command::CommandOptionType,
    interaction::{application_command::CommandDataOptionValue, MessageFlags},
};

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

impl SlashCommandRespond for DocsSlashCommand {
    fn respond<'a, 'b>(
        interaction: &ApplicationCommandInteraction,
        response: &'b mut CreateInteractionResponse<'a>,
    ) -> &'b mut CreateInteractionResponse<'a> {
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
                        .min_by_key(|entry| {
                            strsim::damerau_levenshtein(resolved_option_value, entry.0)
                        })
                        .map(|entry| entry.1)
                        .unwrap();
                }
                option_name => {
                    unreachable!("Unknown option {option_name} of command docs");
                }
            }
        }

        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| {
                data.content(content).flags(MessageFlags::SUPPRESS_EMBEDS)
            })
    }
}

impl SlashCommandAutocomplete for DocsSlashCommand {
    fn autocomplete<'a>(
        interaction: &AutocompleteInteraction,
        autocomplete: &'a mut CreateAutocompleteResponse,
    ) -> &'a mut CreateAutocompleteResponse {
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

                    docs::INDEX
                        .keys()
                        .sorted_by_key(|key| {
                            strsim::damerau_levenshtein(resolved_option_value, key)
                        })
                        .take(10)
                        .for_each(|s| {
                            autocomplete.add_string_choice(s, s);
                        });
                }
                option_name => {
                    unreachable!(
                        "Invalid autocomplete request for option {option_name} of command docs"
                    );
                }
            }
        }

        autocomplete
    }
}
