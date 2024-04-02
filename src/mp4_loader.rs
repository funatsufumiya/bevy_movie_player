use crate::movie::{Player, LoadMode};
use crate::loader::MovieLoader;
use crate::mp4_player::Mp4Movie;

use std::io::BufReader;
use std::time::Duration;

pub struct Mp4Loader;

impl MovieLoader for Mp4Loader {
    fn load(path: &str, load_mode: LoadMode) -> impl Player {
        if load_mode == LoadMode::OnMemory {
            todo!()
        }else{
            let file = std::fs::File::open(path).unwrap();
            // let reader = BufReader::new(file);
            Mp4Movie {
                path: path.to_string(),
                duration: Duration::from_secs(0),
                load_mode,
                audio: None,
                reader: mp4::read_mp4(file).unwrap(),
            }
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let mut movie = Mp4Loader::load("movie.mp4", LoadMode::OnMemory);
        let mut movie = Mp4Loader::load("assets/test.mp4", LoadMode::DiskStream);
        movie.play();
        movie.pause();
        movie.stop();
    }
}

