use bevy::prelude::*;
use gv_video::get_rgba_vec_from_frame;
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
    pub gv: GVVideo<Reader>,
    state: PlayingState,
    play_started_time: Option<Duration>,
    pause_started_time: Option<Duration>,
    seek_position: Duration,
    looped: bool,
}

pub fn load_gv(path: &str, load_mode: LoadMode) -> impl MoviePlayer {
    if load_mode == LoadMode::OnMemory {
        todo!()
    } else {
        let file = std::fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let gv = GVVideo::load(reader);
        
        GVMoviePlayer {
            gv,
            state: PlayingState::Stopped,
            play_started_time: None,
            pause_started_time: None,
            seek_position: Duration::from_secs(0),
            looped: false,
        }
    }
}

impl<Reader: Read + Seek> MoviePlayer for GVMoviePlayer<Reader> {
    fn play(&mut self, looped: bool, bevy_time: &Time) {
        self.looped = looped;
        if self.state == PlayingState::Playing {
            warn!("Already playing");
            return;
        } else if self.state == PlayingState::Paused {
            let paused_duration = bevy_time.elapsed() - self.pause_started_time.unwrap();
            self.play_started_time = Some(self.play_started_time.unwrap() + paused_duration);
            self.pause_started_time = None;
        } else if self.state == PlayingState::Stopped {
            self.play_started_time = Some(bevy_time.elapsed());
        }
        self.state = PlayingState::Playing;
    }

    fn pause(&mut self, bevy_time: &Time) {
        if self.state == PlayingState::Paused {
            warn!("Already paused");
            return;
        } else if self.state == PlayingState::Stopped {
            warn!("Not playing");
            return;
        } else if self.state == PlayingState::Playing {
            self.state = PlayingState::Paused;
            self.pause_started_time = Some(bevy_time.elapsed());
        }
    }

    fn stop(&mut self, _bevy_time: &Time) {
        if self.state == PlayingState::Stopped {
            warn!("Already stopped");
        }

        self.state = PlayingState::Stopped;
        self.seek_position = Duration::from_secs(0);
        self.play_started_time = None;
        self.pause_started_time = None;
    }

    fn seek(&mut self, to_time: Duration, bevy_time: &Time) {
        self.seek_position = to_time;
        self.play_started_time = Some(bevy_time.elapsed());
    }

    fn set_image_data(&mut self, image: &mut Image, bevy_time: &Time) {
        let position = self.get_position(bevy_time);
        if let Ok(frame) = self.gv.read_frame_at(position) {
            let frame_u8 = get_rgba_vec_from_frame(&frame);
            image.data = frame_u8;
        }
    }

    fn get_state(&self) -> PlayingState {
        self.state
    }

    fn get_duration(&self) -> Duration {
        self.gv.get_duration()
    }

    fn update(&mut self, bevy_time: &Time) {
        if self.state == PlayingState::Playing {
            let position = self.get_position(bevy_time);
            if position >= self.get_duration() {
                if self.looped {
                    self.seek(Duration::from_secs(0), bevy_time);
                } else {
                    self.stop(bevy_time);
                }
            }
        }
    }

    fn get_position(&self, bevy_time: &Time) -> Duration {
        match self.state {
            PlayingState::Stopped => Duration::from_secs(0),
            PlayingState::Paused => self.seek_position,
            PlayingState::Playing => (bevy_time.elapsed() - self.play_started_time.unwrap()) + self.seek_position,
        }
    }

    fn set_volume(&mut self, _volume: f32) {
        warn!("Volume is not supported");
        // do nothing
    }

    fn get_volume(&self) -> f32 {
        warn!("Volume is not supported");
        0.0
    }    
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut movie = load_gv("test_assets/test.gv", LoadMode::DiskStream);
        let t = Time::default();
        movie.play(false, &t);
        movie.pause(&t);
        movie.stop(&t);
    }

    // TODO: add duration test
    // TODO: add loop test
    // TODO: add seek test
    // TODO: add image data test
}
