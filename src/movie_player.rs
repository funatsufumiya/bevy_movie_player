use bevy::{prelude::*, render::render_resource::TextureFormat};
use std::time::Duration;

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

pub struct ImageData {
    pub data: Vec<u8>,
    pub format: TextureFormat,
    pub size: (u32, u32),
}

impl ImageData {
    pub fn new(data: Vec<u8>, format: TextureFormat, size: (u32, u32)) -> Self {
        Self {
            data,
            format,
            size,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.size.0
    }

    pub fn get_height(&self) -> u32 {
        self.size.1
    }
}

pub trait MoviePlayer {
    fn play(&mut self, looped: bool, bevy_time: &Time);
    fn pause(&mut self, bevy_time: &Time);
    fn stop(&mut self, bevy_time: &Time);
    fn seek(&mut self, to_time: Duration, bevy_time: &Time);
    fn update(&mut self, bevy_time: &Time);
    fn set_image_data(&mut self, image: &mut Image, bevy_time: &Time);
    fn get_image_data(&mut self, bevy_time: &Time) -> ImageData;
    fn get_state(&self) -> PlayingState;
    fn get_duration(&self) -> Duration;
    fn get_position(&self, bev_time: &Time) -> Duration;
    fn set_volume(&mut self, volume: f32);
    fn get_volume(&self) -> f32;
    fn get_size(&self) -> (u32, u32);
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