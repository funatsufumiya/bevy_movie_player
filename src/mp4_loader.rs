use crate::movie::{Player, LoadMode};
use crate::loader::MovieLoader;
use crate::mp4_player::Mp4Movie;

use std::time::Duration;

pub struct Mp4Loader;

impl MovieLoader for Mp4Loader {
    fn load(path: &str, load_mode: LoadMode) -> impl Player {
        Mp4Movie {
            path: path.to_string(),
            duration: Duration::from_secs(0),
            load_mode,
            audio: None,
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use crate::{loader::MovieLoader, mp4_loader::Mp4Loader};
    use crate::movie::Player;

    use super::*;

    #[test]
    fn it_works() {
        let mut movie = Mp4Loader::load("movie.mp4", LoadMode::OnMemory);
        movie.play();
        movie.pause();
        movie.stop();
    }
}

