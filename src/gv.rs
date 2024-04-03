use gv_video::GVVideo;

use crate::movie_player::LoadMode;
use crate::movie_player::PlayingState;
use crate::movie_player::MoviePlayer;

use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::time::Duration;

#[derive(Debug)]
pub struct GVMoviePlayer<Reader: Read + Seek> {
    pub gv: GVVideo<Reader>
}

pub fn load_gv(path: &str, load_mode: LoadMode) -> impl MoviePlayer {
    if load_mode == LoadMode::OnMemory {
        todo!()
    } else {
        let file = std::fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let gv = GVVideo::load(reader);
        
        GVMoviePlayer {
            gv
        }
    }
}

impl<Reader: Read + Seek> MoviePlayer for GVMoviePlayer<Reader> {
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
        let mut movie = load_gv("assets/test.gv", LoadMode::DiskStream);
        movie.play();
        movie.pause();
        movie.stop();
    }
}
