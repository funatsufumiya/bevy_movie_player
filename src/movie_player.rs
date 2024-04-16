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

#[allow(non_camel_case_types)]
/// BlankMode is used for blanking the screen when the movie is not playing.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BlankMode {
    Black,
    White,
    Transparent,
    /// use last frame on pause, otherwise transparent
    LastFrameOnPause_TransparentOnStop,
    /// use last frame on pause, use first frame on stop
    LastFrameOnPause_FirstFrameOnStop,
    /// use last frame on pause and stop
    LastFrameOnPauseAndStop,
}

impl Default for BlankMode {
    fn default() -> Self {
        BlankMode::LastFrameOnPause_FirstFrameOnStop
    }
}

pub trait MoviePlayer {
    fn play(&mut self);
    fn pause(&mut self);
    fn stop(&mut self);
    fn seek(&mut self, to_time: Duration);
    fn update(&mut self, bevy_elapsed_time: Duration);
    fn get_state(&self) -> PlayingState;
    fn get_duration(&self) -> Duration;
    fn get_position(&self) -> Duration;
    fn set_volume(&mut self, volume: f32);
    fn get_volume(&self) -> f32;
    fn get_resolution(&self) -> (u32, u32);
    fn get_loop_mode(&self) -> LoopMode;
    fn set_loop_mode(&mut self, loop_mode: LoopMode);
}

pub trait Blankable {
    fn set_blank_mode(&mut self, blank_mode: BlankMode);
    fn get_blank_mode(&self) -> BlankMode;
}

pub trait ImageDataProvider {
    /// set image data to image with uncompressed texture format (like BGRA8UnormSrgb)
    fn set_image_data(&mut self, image: &mut Image);
    /// returns image data with uncompressed texture format (like BGRA8UnormSrgb)
    fn get_image_data(&mut self) -> ImageData;
}

pub trait CompressedImageDataProvider {
    /// set image data to image with compressed texture format (like BC7Srgb)
    fn set_compressed_image_data(&mut self, image: &mut Image);
    /// returns image data with compressed texture format (like BC7Srgb)
    fn get_compressed_image_data(&mut self) -> ImageData;
}

pub trait ImageCreator {
    fn create_image(&mut self) -> Image;
    fn register_image_handle(&mut self, images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
        let mut image = self.create_image();
        let handle = images.add(image);
        return handle;
    }
}

impl<U> ImageCreator for U
where U: ImageDataProvider
{
    fn create_image(&mut self) -> Image {
        let image_data = self.get_image_data();

        let image = Image::new(
            Extent3d {
                width: image_data.get_width(),
                height: image_data.get_height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            image_data.data,
            image_data.format,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
    
        return image; 
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