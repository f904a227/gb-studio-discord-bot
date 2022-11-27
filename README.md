[![Continuous integration](https://github.com/f904a227/gb-studio-discord-bot/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/f904a227/gb-studio-discord-bot/actions/workflows/ci.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

# GB Studio Discord bot

## Using the bot

The bot requires at least the "Manage Roles" and "Send Messages" permissions (`268437504`), and the ability to create app commands (`applications.commands`) and to run as a bot user (`bot`).

1. Create a new application on the [Discord Developer Portal](https://discord.com/developers) and add a bot.
2. Invite the bot to your server using this link: `https://discord.com/api/oauth2/authorize?client_id=YOUR-APP-ID-GOES-HERE&permissions=268437504&scope=bot%20applications.commands`; replace `YOUR-APP-ID-GOES-HERE` by your application ID.
3. Clone the repository.
4. Run `DISCORD_BOT_TOKEN='YOUR-TOKEN-GOES-HERE' DISCORD_GUILD_ID='YOUR-SERVER-ID-GOES-HERE' cargo run --release`; replace `YOUR-TOKEN-GOES-HERE` by your bot token and `YOUR-SERVER-ID-GOES-HERE` by your server ID.

## Contributing to the bot

1. Clone the repository.
2. Run `pre-commit install && pre-commit install --hook-type commit-msg`.
3. Branch from `main`.
4. Code!
5. Run `cargo check`.
7. Submit a pull request to merge back into `main`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
