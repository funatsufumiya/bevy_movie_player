use mp4::parse::Mp4File;

use crate::movie_player::LoadMode;
use crate::movie_player::PlayingState;
use crate::movie_player::MoviePlayer;

use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::time::Duration;
use std::fmt;

impl<Reader: Read + Seek> fmt::Debug for Mp4Movie<Reader> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mp4Movie")
            .field("duration", &self.duration)
            .field("load_mode", &self.load_mode)
            .field("mp4", &self.mp4)
            // .field("reader",  &"Reader")
            .finish()
    }
}
pub struct Mp4Movie<Reader: Read + Seek> {
    pub duration: Duration,
    pub load_mode: LoadMode,
    pub mp4: Mp4File,
    pub reader: Reader,
}

pub fn load_mp4(path: &str, load_mode: LoadMode) -> impl MoviePlayer {
    if load_mode == LoadMode::OnMemory {
        todo!()
    } else {
        let file = std::fs::File::open(path).unwrap();
        let mp4 = mp4::parse::parse(file.try_clone().unwrap()).unwrap();
        let reader = BufReader::new(file);

        Mp4Movie {
            duration: Duration::from_secs(0),
            load_mode,
            mp4,
            reader: reader,
        }
    }
}

impl<Reader: Read + Seek> MoviePlayer for Mp4Movie<Reader> {
    fn play(&mut self) {
        todo!()
    }

    fn pause(&mut self) {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }

    fn seek(&self, time: Duration) {
        todo!()
    }
    
    fn get_state(&self) -> PlayingState {
        todo!()
    }
    
    fn set_state(&mut self, state: PlayingState) {
        todo!()
    }
    
    fn get_duration(&self) -> Duration {
        todo!()
    }
    
    fn get_position(&self) -> Duration {
        todo!()
    }
    
    fn set_volume(&mut self, volume: f32) {
        todo!()
    }
    
    fn get_volume(&self) -> f32 {
        todo!()
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let mut movie = Mp4Loader::load("movie.mp4", LoadMode::OnMemory);
        let mut movie = load_mp4("assets/test.mp4", LoadMode::DiskStream);
        movie.play();
        movie.pause();
        movie.stop();
    }
}
