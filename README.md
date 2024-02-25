# drp-rs - Discord Rich Presence
Discord Rich Presence is a tool that lets you update your Discord playing status to whatever is playing on your computer, or even things like Plex (https://github.com/Tautulli/Tautulli) [via Tautulli]. The Rust code isn't too professional yet, as it's a project that just started. Linux and Windows support are both offered.
NOTE - still in development. 

|   OS      | Supported? |
|    ---    |   ---      |
|   Linux   |    yes     |
|   Windows |    yes     |
|   Mac     | eventually?|

## Planned features
Currently, support is planned to be added for an extension (which will change a lot of how drp will work), and Jellyfin session support is also planned. If TIDAL comes up with a special API, it will be incorporated since Mac doesn't seem to like apps looking for Plex/Tautulli support will be improved, and a GUI might be added.

## Config
Copy the `App.example.toml` file into `App.toml`. Filling all the config options is required, even if you're only using the local media reporting.
