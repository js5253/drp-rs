# drp-rs - Discord Rich Presence
## Architecture & Project Structure
The bulk of the app is contained in the main.rs file, but metadata providers are separated into their respective files. Linux support is offered through MprisProvider.rs, while the Windows support uses the Windows crate. A small webserver with Axum is offered in case you want to use the extension (found in the "extension" directory.) This project largely uses Axum to handle concurrency, along with a bevy of other features (cancellation tokens, spawn_local,  etc)


Discord Rich Presence is a tool that lets you update your Discord playing status to whatever is playing on your computer, or even things like Plex (https://github.com/Tautulli/Tautulli) [via Tautulli]. The Rust code isn't too professional yet, as it's my first serious Rust project. Linux and Windows support are both offered.
Binaries aren't available at the moment, but they will eventually be.

|   OS      | Supported? |
|    ---    |   ---      |
|   Linux   |    yes     |
|   Windows |    yes     |

## Planned features
This project is considered finished save for a few things: CI/CD, adding tests, improving the UI. If anybody wants to add more websites (currently it has support for YouTube Music) they are welcome.

