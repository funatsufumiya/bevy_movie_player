use std::io::{Read, Seek};

use bevy::{image::Image, render::render_resource::{Extent3d, TextureFormat}};

use crate::{image_data_provider::{CompressedImageDataProvider, ImageDataProvider}, movie_player::{ImageData, PlayingState}, prelude::MoviePlayer};

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

pub trait BGRAImageFrameProvider {
    fn get_first_frame_bgra(&mut self) -> Option<Vec<u8>>;
    fn get_paused_frame_bgra(&mut self) -> Option<Vec<u8>>;
    fn get_playing_frame_bgra(&mut self) -> Option<Vec<u8>>;
    fn get_last_frame_bgra(&mut self) -> Option<Vec<u8>>;
}

pub trait RGBImageFrameProvider {
    fn get_first_frame_rgb(&mut self) -> Option<Vec<u8>>;
    fn get_paused_frame_rgb(&mut self) -> Option<Vec<u8>>;
    fn get_playing_frame_rgb(&mut self) -> Option<Vec<u8>>;
    fn get_last_frame_rgb(&mut self) -> Option<Vec<u8>>;
}

pub trait CompressedImageFrameProvider {
    fn get_first_frame_compressed(&mut self) -> Option<Vec<u8>>;
    fn get_paused_frame_compressed(&mut self) -> Option<Vec<u8>>;
    fn get_playing_frame_compressed(&mut self) -> Option<Vec<u8>>;
    fn get_last_frame_compressed(&mut self) -> Option<Vec<u8>>;
    fn get_texture_format(&self) -> TextureFormat;
}

pub trait BlankableBGRA {
    fn set_blank_mode(&mut self, blank_mode: BlankMode);
    fn get_blank_mode(&self) -> BlankMode;

    fn black_frame_bgra_1x1() -> &'static [u8] {
        &[0, 0, 0, 255]
    }

    fn white_frame_bgra_1x1() -> &'static [u8] {
        &[255, 255, 255, 255]
    }

    fn transparent_frame_bgra_1x1() -> &'static [u8] {
        &[0, 0, 0, 0]
    }

    fn texture_1x1_bgra(data: &[u8]) -> ImageData {
        ImageData {
            data: data.to_vec(),
            format: TextureFormat::Bgra8UnormSrgb,
            resolution: (1, 1),
        }
    }

    // fn texture_bgra(data: &Vec<u8>, width: u32, height: u32) -> ImageData {
    //     ImageData {
    //         data: data.clone(),
    //         format: TextureFormat::Bgra8UnormSrgb,
    //         resolution: (width, height),
    //     }
    // }

    fn get_blank_frame_bgra(&self, state: PlayingState, last_or_first_frame: Option<ImageData>) -> ImageData {
        match self.get_blank_mode() {
            BlankMode::Black => Self::texture_1x1_bgra(Self::black_frame_bgra_1x1()),
            BlankMode::White => Self::texture_1x1_bgra(Self::white_frame_bgra_1x1()),
            BlankMode::Transparent => Self::texture_1x1_bgra(Self::transparent_frame_bgra_1x1()),
            BlankMode::LastFrameOnPause_TransparentOnStop => {
                if state == PlayingState::Paused {
                    if let Some(last_frame) = last_or_first_frame {
                        last_frame
                    } else {
                        Self::texture_1x1_bgra(Self::transparent_frame_bgra_1x1())
                    }
                } else {
                    Self::texture_1x1_bgra(Self::transparent_frame_bgra_1x1())
                }
            },
            BlankMode::LastFrameOnPause_FirstFrameOnStop => {
                if state == PlayingState::Paused {
                    if let Some(last_frame) = last_or_first_frame {
                        last_frame
                    } else {
                        Self::texture_1x1_bgra(Self::transparent_frame_bgra_1x1())
                    }
                } else if state == PlayingState::Stopped {
                    if let Some(first_frame) = last_or_first_frame {
                        first_frame
                    } else {
                        Self::texture_1x1_bgra(Self::transparent_frame_bgra_1x1())
                    }
                } else {
                    Self::texture_1x1_bgra(Self::transparent_frame_bgra_1x1())
                }
            },
            BlankMode::LastFrameOnPauseAndStop => {
                if let Some(last_or_first_frame) = last_or_first_frame {
                    last_or_first_frame
                } else {
                    Self::texture_1x1_bgra(Self::transparent_frame_bgra_1x1())
                }
            },
        }
    }
}

pub trait BlankableRGB {
    fn set_blank_mode(&mut self, blank_mode: BlankMode);
    fn get_blank_mode(&self) -> BlankMode;

    fn black_frame_rgb_1x1() -> &'static [u8] {
        &[0, 0, 0]
    }
    fn white_frame_rgb_1x1() -> &'static [u8] {
        &[255, 255, 255]
    }
    fn transparent_frame_rgb_1x1() -> &'static [u8] {
        &[0, 0, 0]
    }
    fn texture_1x1_rgb(data: &[u8]) -> ImageData {
        ImageData {
            data: data.to_vec(),
            format: TextureFormat::Rgb8Unorm,
            resolution: (1, 1),
        }
    }
    // fn texture_rgb(data: &Vec<u8>, width: u32, height: u32) -> ImageData {
    //     ImageData {
    //         data: data.clone(),
    //         format: TextureFormat::Rgb8Unorm,
    //         resolution: (width, height),
    //     }
    // }
    fn get_blank_frame_rgb(&self, state: PlayingState, last_or_first_frame: Option<ImageData>) -> ImageData {
        match self.get_blank_mode() {
            BlankMode::Black => Self::texture_1x1_rgb(Self::black_frame_rgb_1x1()),
            BlankMode::White => Self::texture_1x1_rgb(Self::white_frame_rgb_1x1()),
            BlankMode::Transparent => Self::texture_1x1_rgb(Self::transparent_frame_rgb_1x1()),
            BlankMode::LastFrameOnPause_TransparentOnStop => {
                if state == PlayingState::Paused {
                    if let Some(last_frame) = last_or_first_frame {
                        last_frame
                    } else {
                        Self::texture_1x1_rgb(Self::transparent_frame_rgb_1x1())
                    }
                } else {
                    Self::texture_1x1_rgb(Self::transparent_frame_rgb_1x1())
                }
            },
            BlankMode::LastFrameOnPause_FirstFrameOnStop => {
                if state == PlayingState::Paused {
                    if let Some(last_frame) = last_or_first_frame {
                        last_frame
                    } else {
                        Self::texture_1x1_rgb(Self::transparent_frame_rgb_1x1())
                    }
                } else if state == PlayingState::Stopped {
                    if let Some(first_frame) = last_or_first_frame {
                        first_frame
                    } else {
                        Self::texture_1x1_rgb(Self::transparent_frame_rgb_1x1())
                    }
                } else {
                    Self::texture_1x1_rgb(Self::transparent_frame_rgb_1x1())
                }
            },
            BlankMode::LastFrameOnPauseAndStop => {
                if let Some(last_or_first_frame) = last_or_first_frame {
                    last_or_first_frame
                } else {
                    Self::texture_1x1_rgb(Self::transparent_frame_rgb_1x1())
                }
            },
        }
    }
}

impl<T> ImageDataProvider for T
where
    T: BlankableBGRA + MoviePlayer + BGRAImageFrameProvider
{
    fn get_image_data(&mut self) -> ImageData {
        match self.get_state() {
            PlayingState::Stopped => {
                // FIXME: slow? need cached for first and last frame?
                let frame_or_not = if self.get_blank_mode() == BlankMode::LastFrameOnPause_FirstFrameOnStop {
                    self.get_first_frame_bgra()
                } else if self.get_blank_mode() == BlankMode::LastFrameOnPauseAndStop {
                    self.get_last_frame_bgra()
                } else {
                    None
                };
                let frame_data = if let Some(frame) = frame_or_not {
                    Some(ImageData {
                        data: frame,
                        format: TextureFormat::Bgra8UnormSrgb,
                        resolution: self.get_resolution(),
                    })
                } else {
                    None
                };
                
                self.get_blank_frame_bgra(self.get_state(), frame_data)
            }
            PlayingState::Paused => {
                // FIXME: slow? need cached for first and last frame?
                let last_frame = self.get_paused_frame_bgra();
                let last_frame_data = if let Some(frame) = last_frame {
                    Some(ImageData {
                        data: frame,
                        format: TextureFormat::Bgra8UnormSrgb,
                        resolution: self.get_resolution(),
                    })
                } else {
                    None
                };
                
                self.get_blank_frame_bgra(self.get_state(), last_frame_data)
            }
            PlayingState::Playing => {
                let frame = self.get_playing_frame_bgra();
                let frame_data = if let Some(frame) = frame {
                    ImageData {
                        data: frame,
                        format: TextureFormat::Bgra8UnormSrgb,
                        resolution: self.get_resolution(),
                    }
                } else {
                    // WORKAROUND
                    self.get_blank_frame_bgra(self.get_state(), None)
                };
                frame_data
            }
        }
    }
}

impl<T> CompressedImageDataProvider for T
where
    T: BlankableBGRA + MoviePlayer + CompressedImageFrameProvider
{
    fn get_compressed_image_data(&mut self) -> ImageData {
        match self.get_state() {
            PlayingState::Stopped => {
                // FIXME: slow? need cached for first and last frame?
                let frame_or_not = if self.get_blank_mode() == BlankMode::LastFrameOnPause_FirstFrameOnStop {
                    self.get_first_frame_compressed()
                } else if self.get_blank_mode() == BlankMode::LastFrameOnPauseAndStop {
                    self.get_last_frame_compressed()
                } else {
                    None
                };
                let frame_data = if let Some(frame) = frame_or_not {
                    Some(ImageData {
                        data: frame,
                        format: self.get_texture_format(),
                        resolution: self.get_resolution(),
                    })
                } else {
                    None
                };
                self.get_blank_frame_bgra(self.get_state(), frame_data)
            }
            PlayingState::Paused => {
                // FIXME: slow? need cached for first and last frame?
                let last_frame = self.get_paused_frame_compressed();
                let last_frame_data = if let Some(frame) = last_frame {
                    Some(ImageData {
                        data: frame,
                        format: self.get_texture_format(),
                        resolution: self.get_resolution(),
                    })
                } else {
                    None
                };
                self.get_blank_frame_bgra(self.get_state(), last_frame_data)
            }
            PlayingState::Playing => {
                let frame = self.get_playing_frame_compressed();
                let frame_data = if let Some(frame) = frame {
                    ImageData {
                        data: frame,
                        format: self.get_texture_format(),
                        resolution: self.get_resolution(),
                    }
                } else {
                    // WORKAROUND
                    self.get_blank_frame_bgra(self.get_state(), None)
                };
                frame_data
            }
        }
    }
}