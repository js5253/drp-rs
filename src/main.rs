use mpris::{PlayerFinder, Player};

struct PlayingMetadata {
    name: String,
    artist: String,
}
fn get_playing_metadata(player: &Option<Player>) -> Option<PlayingMetadata> {
    match player {
        Ok(player) => {
            let metadata = player.get_metadata().unwrap().as_hashmap();

            let title = metadata.get("fs:title").unwrap().as_str();
            let artist = metadata.get("fs:artist").unwrap().as_str_array()[0];

            let playing = PlayingMetadata { name, artist };

            Some(playing)
        }
        Err(_) => None, // can't find any player
    }
}

fn main() {
    let player = PlayerFinder::new().find_first(&self);
    let prev_playing = get_playing_metadata(&player);
    println!("", prev_playing);
    loop {
        let playing = get_playing_metadata(&player);

        match playing {
            Some(metadata) => {
                if prev_playing == playing {
                    println!("Now playing {} by {}", metadata.name, metadata.artist)
                }
            }
            None => println!("Not playing anything..."),
        }
    }
}
