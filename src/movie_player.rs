use bevy::{asset::AssetLoader, prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}};
use std::{fmt, time::Duration};

// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum LoadMode {
//     OnMemory,
//     DiskStream,
// }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlayingState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LoopMode {
    Stop,
    Loop,
    PauseAtEnd,
}

impl Default for LoopMode {
    fn default() -> Self {
        LoopMode::Stop
    }
}

// #[derive(Clone)]
// #[derive(Debug)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub format: TextureFormat,
    pub resolution: (u32, u32),
}

impl fmt::Debug for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageData")
            .field("data", &format!("{} bytes", self.data.len()))
            .field("format", &self.format)
            .field("resolution", &self.resolution)
            .finish()
    }
}

impl ImageData {
    pub fn new(data: Vec<u8>, format: TextureFormat, resolution: (u32, u32)) -> Self {
        Self {
            data,
            format,
            resolution,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.resolution.0
    }

    pub fn get_height(&self) -> u32 {
        self.resolution.1
    }
}

#[derive(Debug, Clone)]
pub struct SeekOutOfBoundsError {
    pub actual_seeked_position: Duration,
}

pub struct MoviePlayerStateController {
    pub state: PlayingState,
    pub bevy_elapsed_time: Duration,
    pub play_started_time: Option<Duration>,
    pub pause_started_time: Option<Duration>,
    pub seek_position: Duration,
    pub loop_mode: LoopMode,
}

impl Default for MoviePlayerStateController {
    fn default() -> Self {
        Self {
            state: PlayingState::Stopped,
            bevy_elapsed_time: Duration::from_secs(0),
            play_started_time: None,
            pause_started_time: None,
            seek_position: Duration::from_secs(0),
            loop_mode: LoopMode::default(),
        }
    }
}

impl MoviePlayerStateController {
    pub fn play(&mut self, bevy_elapsed_time: Duration) {
        if self.state == PlayingState::Playing {
            warn!("Already playing");
            return;
        } else if self.state == PlayingState::Paused {
            let paused_duration = bevy_elapsed_time - self.pause_started_time.unwrap();
            self.play_started_time = Some(self.play_started_time.unwrap() + paused_duration);
            self.pause_started_time = None;
        } else if self.state == PlayingState::Stopped {
            self.play_started_time = Some(bevy_elapsed_time);
        }
        self.state = PlayingState::Playing;
    }

    pub fn pause(&mut self, bevy_elapsed_time: Duration) {
        if self.state == PlayingState::Paused {
            warn!("Already paused");
            return;
        } else if self.state == PlayingState::Stopped {
            warn!("Not playing");
            return;
        } else if self.state == PlayingState::Playing {
            self.state = PlayingState::Paused;
            self.pause_started_time = Some(bevy_elapsed_time);
            self.seek_position = (bevy_elapsed_time - self.play_started_time.unwrap()) + self.seek_position;
            self.play_started_time = self.pause_started_time;
        }
    }

    pub fn stop(&mut self) {
        if self.state == PlayingState::Stopped {
            warn!("Already stopped");
        }
        self.state = PlayingState::Stopped;
        self.seek_position = Duration::from_secs(0);
        self.play_started_time = None;
        self.pause_started_time = None;
    }

    pub fn seek(&mut self, to_time: Duration, bevy_elapsed_time: Duration, movie_total_duration: Duration)  -> Result<Duration, SeekOutOfBoundsError> {
        if to_time < Duration::from_secs(0) {
            self.seek_position = Duration::from_secs(0);
            self.play_started_time = Some(bevy_elapsed_time);
            return Err(SeekOutOfBoundsError {
                actual_seeked_position: Duration::from_secs(0),
            });
        }
        if to_time > movie_total_duration {
            // // WORKAROUND: seek to the end - 0.1ms
            // let actual_to_time = movie_total_duration - Duration::from_secs_f32(0.0001);
            // self.seek_position = actual_to_time;
            // self.play_started_time = Some(bevy_elapsed_time);
            // return Err(SeekOutOfBoundsError {
            //     actual_seeked_position: actual_to_time,
            // });

            let actual_to_time = movie_total_duration;
            self.seek_position = actual_to_time;
            self.play_started_time = Some(bevy_elapsed_time);
            return Err(SeekOutOfBoundsError {
                actual_seeked_position: actual_to_time,
            });
        }
        self.seek_position = to_time;
        self.play_started_time = Some(bevy_elapsed_time);
        Ok(self.seek_position)
    }

    pub fn get_state(&self) -> PlayingState {
        self.state
    }

    pub fn get_position(&self, bevy_elapsed_time: Duration) -> Duration {
        match self.state {
            PlayingState::Stopped => Duration::from_secs(0),
            PlayingState::Paused => self.seek_position,
            PlayingState::Playing => (bevy_elapsed_time - self.play_started_time.unwrap()) + self.seek_position,
        }
    }

    pub fn update(&mut self, bevy_elapsed_time: Duration, duration: Duration) {
        self.bevy_elapsed_time = bevy_elapsed_time;

        if self.state == PlayingState::Playing {
            let position = self.get_position(bevy_elapsed_time);
            if position >= duration {
                match self.loop_mode {
                    LoopMode::Stop => {
                        self.stop();
                    },
                    LoopMode::Loop => {
                        let _ = self.seek(Duration::from_secs(0), bevy_elapsed_time, duration);
                    },
                    LoopMode::PauseAtEnd => {
                        // WORKAROUND: seek to the end - 0.1ms
                        let _ = self.seek(duration - Duration::from_secs_f32(0.0001), bevy_elapsed_time, duration);
                        self.state = PlayingState::Paused;
                    },
                }
            }
        }
    }

    pub fn get_loop_mode(&self) -> LoopMode {
        self.loop_mode
    }
    
    pub fn set_loop_mode(&mut self, loop_mode: LoopMode) {
        self.loop_mode = loop_mode;
    }

}

pub trait MoviePlayer {
    fn get_state_controller(&self) -> &MoviePlayerStateController;
    fn get_state_controller_mut(&mut self) -> &mut MoviePlayerStateController;
    fn get_duration(&self) -> Duration;
    fn get_resolution(&self) -> (u32, u32);
    fn set_volume(&mut self, _volume: f32);
    fn get_volume(&self) -> f32;

    fn play(&mut self) {
        let state_controller = self.get_state_controller_mut();
        state_controller.play(state_controller.bevy_elapsed_time);
    }

    fn pause(&mut self) {
        let state_controller = self.get_state_controller_mut();
        state_controller.pause(state_controller.bevy_elapsed_time);
    }

    fn stop(&mut self) {
        self.get_state_controller_mut().stop();
    }

    fn seek(&mut self, to_time: Duration) -> Result<Duration, SeekOutOfBoundsError> {
        let duration = self.get_duration();
        let state_controller = self.get_state_controller_mut();
        state_controller.seek(to_time, state_controller.bevy_elapsed_time, duration)
    }

    fn get_state(&self) -> PlayingState {
        self.get_state_controller().get_state()
    }

    fn update(&mut self, bevy_elapsed_time: Duration) {
        let duration = self.get_duration();
        let state_controller = self.get_state_controller_mut();
        state_controller.update(bevy_elapsed_time, duration);
    }

    fn get_position(&self) -> Duration {
        let state_controller = self.get_state_controller();
        state_controller.get_position(state_controller.bevy_elapsed_time)
    }
    
    fn get_loop_mode(&self) -> LoopMode {
        self.get_state_controller().get_loop_mode()
    }
    
    fn set_loop_mode(&mut self, loop_mode: LoopMode) {
        self.get_state_controller_mut().set_loop_mode(loop_mode);
    }
}

pub trait StateChecker {
    fn is_playing(&self) -> bool;
    fn is_paused(&self) -> bool;
    fn is_stopped(&self) -> bool;
}

impl<T: MoviePlayer> StateChecker for T {
    fn is_playing(&self) -> bool {
        self.get_state() == PlayingState::Playing
    }

    fn is_paused(&self) -> bool {
        self.get_state() == PlayingState::Paused
    }

    fn is_stopped(&self) -> bool {
        self.get_state() == PlayingState::Stopped
    }
}