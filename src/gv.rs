use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AssetPath;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::log::tracing_subscriber::field::debug;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::utils::BoxedFuture;
use bevy::utils::ConditionalSendFuture;
use derivative::Derivative;
use gv_video::get_bgra_vec_from_frame;
use gv_video::GVVideo;
use gv_video::GVFormat;

use crate::movie_player::BlankMode;
use crate::movie_player::Blankable;
use crate::movie_player::CompressedImageDataProvider;
use crate::movie_player::ImageData;
use crate::movie_player::ImageDataProvider;
use crate::movie_player::LoopMode;
use crate::movie_player::MoviePlayerStateController;
// use crate::movie_player::LoadMode;
use crate::movie_player::PlayingState;
use crate::movie_player::MoviePlayer;

use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::time::Duration;
use bevy::prelude::TypePath;

pub struct GVMoviePlayer<Reader: Read + Seek> {
    pub gv: GVVideo<Reader>,
    blank_mode: BlankMode,
    state_controller: MoviePlayerStateController,
}

#[derive(Asset, TypePath, Derivative)]
#[derivative(Debug)]
pub struct GVMovie {
    #[derivative(Debug="ignore")]
    pub player: GVMoviePlayer<BufReader<File>>,
}

#[derive(Asset, TypePath, Derivative)]
#[derivative(Debug)]
pub struct GVMovieOnMemory {
    #[derivative(Debug="ignore")]
    pub player: GVMoviePlayer<Cursor<Vec<u8>>>,
}

#[derive(Default)]
pub struct GVMovieLoader;

#[derive(Default)]
pub struct GVMovieOnMemoryLoader;

impl AssetLoader for GVMovieLoader {
    type Asset = GVMovie;
    type Settings = ();
    type Error = std::io::Error;
  
    fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
      Box::pin(async move {
        let asset_dir = "assets"; // FIXME: just WORKAROUND
        let p = Path::new(asset_dir).join(load_context.path());
        let asset_path = p.to_str().unwrap();
        // TODO: error handling
        let player = load_gv(asset_path);
        Ok(GVMovie {
          player,
        })
      })
    }
  
    fn extensions(&self) -> &[&str] {
      &["gv"]
    }
}

impl AssetLoader for GVMovieOnMemoryLoader {
    type Asset = GVMovieOnMemory;
    type Settings = ();
    type Error = std::io::Error;
  
    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
      Box::pin(async move {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        // TODO: error handling
        let player = load_gv_from_reader(Cursor::new(bytes));
        Ok(GVMovieOnMemory {
          player,
        })
      })
    }
  
    fn extensions(&self) -> &[&str] {
      &["gv"]
    }
}

/// Load a GV video from a file (disk stream)
pub fn load_gv(path: &str) -> GVMoviePlayer<BufReader<File>> {
    let file = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(file);
    let gv = GVVideo::load(reader);
    
    GVMoviePlayer {
        gv,
        blank_mode: BlankMode::default(),
        state_controller: MoviePlayerStateController::default(),
    }
}

/// Load a GV video from a reader
pub fn load_gv_from_reader<R>(reader: R) -> GVMoviePlayer<R>
    where R: Read + Seek
{
    let gv = GVVideo::load(reader);
    
    GVMoviePlayer {
        gv,
        blank_mode: BlankMode::default(),
        state_controller: MoviePlayerStateController::default(),
    }
}

/// Load a GV video from a file (on memory)
pub fn load_gv_on_memory(path: &str) -> GVMoviePlayer<Cursor<Vec<u8>>> {
    let file = File::open(path).unwrap();
    // load all data into memory
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    // use cursor
    let reader = Cursor::new(buffer);
    let gv = GVVideo::load(reader);
    
    GVMoviePlayer {
        gv,
        blank_mode: BlankMode::default(),
        state_controller: MoviePlayerStateController::default(),
    }
}

impl<Reader: Read + Seek> Blankable for GVMoviePlayer<Reader> {
    fn get_blank_mode(&self) -> BlankMode {
        self.blank_mode
    }
    
    fn set_blank_mode(&mut self, blank_mode: BlankMode) {
        self.blank_mode = blank_mode;
    }
}

impl<Reader: Read + Seek> MoviePlayer for GVMoviePlayer<Reader> {
    fn get_state_controller(&self) -> &crate::movie_player::MoviePlayerStateController {
        &self.state_controller
    }

    fn get_state_controller_mut(&mut self) -> &mut crate::movie_player::MoviePlayerStateController {
        &mut self.state_controller
    }

    fn get_duration(&self) -> Duration {
        self.gv.get_duration()
    }

    fn get_resolution(&self) -> (u32, u32) {
        self.gv.get_resolution()
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

const fn black_frame_bgra_1x1() -> &'static [u8] {
    &[0, 0, 0, 255]
}

const fn white_frame_bgra_1x1() -> &'static [u8] {
    &[255, 255, 255, 255]
}

const fn transparent_frame_bgra_1x1() -> &'static [u8] {
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

fn get_blank_frame_bgra(blank_mode: BlankMode, state:PlayingState, last_or_first_frame: Option<ImageData>) -> ImageData {
    // NOTE: BGRA format, black is (0, 0, 0, 255), white is (255, 255, 255, 255)
    match blank_mode {
        BlankMode::Black => texture_1x1_bgra(black_frame_bgra_1x1()),
        BlankMode::White => texture_1x1_bgra(white_frame_bgra_1x1()),
        BlankMode::Transparent => texture_1x1_bgra(transparent_frame_bgra_1x1()),
        BlankMode::LastFrameOnPause_TransparentOnStop => {
            if state == PlayingState::Paused {
                if let Some(last_frame) = last_or_first_frame {
                    last_frame
                } else {
                    texture_1x1_bgra(transparent_frame_bgra_1x1())
                }
            } else {
                texture_1x1_bgra(transparent_frame_bgra_1x1())
            }
        },
        BlankMode::LastFrameOnPause_FirstFrameOnStop => {
            if state == PlayingState::Paused {
                if let Some(last_frame) = last_or_first_frame {
                    last_frame
                } else {
                    texture_1x1_bgra(transparent_frame_bgra_1x1())
                }
            } else if state == PlayingState::Stopped {
                if let Some(first_frame) = last_or_first_frame {
                    first_frame
                } else {
                    texture_1x1_bgra(transparent_frame_bgra_1x1())
                }
            } else {
                texture_1x1_bgra(transparent_frame_bgra_1x1())
            }
        },
        BlankMode::LastFrameOnPauseAndStop => {
            if let Some(last_or_first_frame) = last_or_first_frame {
                last_or_first_frame
            } else {
                texture_1x1_bgra(transparent_frame_bgra_1x1())
            }
        },
    }
}

impl<Reader: Read + Seek> ImageDataProvider for GVMoviePlayer<Reader> {
    fn set_image_data(&mut self, image: &mut Image) {
        let image_data = self.get_image_data();
        image.data = image_data.data;
        image.texture_descriptor.format = image_data.format;
        image.texture_descriptor.size = Extent3d {
            width: image_data.resolution.0,
            height: image_data.resolution.1,
            depth_or_array_layers: 1,
        };
    }

    fn get_image_data(&mut self) -> ImageData {
        match self.get_state() {
            PlayingState::Stopped => {
                // FIXME: slow? need cached for first and last frame?
                let frame_or_not = if self.get_blank_mode() == BlankMode::LastFrameOnPause_FirstFrameOnStop {
                    self.gv.read_frame(0).ok()
                } else if self.get_blank_mode() == BlankMode::LastFrameOnPauseAndStop {
                    self.gv.read_frame(self.gv.get_frame_count() - 1).ok()
                } else {
                    None
                };
                let frame_data = if let Some(frame) = frame_or_not {
                    Some(ImageData {
                        data: get_bgra_vec_from_frame(frame),
                        format: TextureFormat::Bgra8UnormSrgb,
                        resolution: self.gv.get_resolution(),
                    })
                } else {
                    None
                };
                
                get_blank_frame_bgra(self.get_blank_mode(), self.get_state(), frame_data)
            }
            PlayingState::Paused => {
                // FIXME: slow? need cached for first and last frame?
                let last_frame = self.gv.read_frame_at(self.get_position()).ok();
                let last_frame_data = if let Some(frame) = last_frame {
                    Some(ImageData {
                        data: get_bgra_vec_from_frame(frame),
                        format: TextureFormat::Bgra8UnormSrgb,
                        resolution: self.gv.get_resolution(),
                    })
                } else {
                    None
                };
                
                get_blank_frame_bgra(self.get_blank_mode(), self.get_state(), last_frame_data)
            }
            PlayingState::Playing => {
                let frame = self.gv.read_frame_at(self.get_position()).ok();
                let frame_data = if let Some(frame) = frame {
                    ImageData {
                        data: get_bgra_vec_from_frame(frame),
                        format: TextureFormat::Bgra8UnormSrgb,
                        resolution: self.gv.get_resolution(),
                    }
                } else {
                    // WORKAROUND
                    get_blank_frame_bgra(self.get_blank_mode(), self.get_state(), None)
                };
                frame_data
            }
        }
    }
}

fn get_texture_format(gv_format: GVFormat) -> TextureFormat {
    match gv_format {
        GVFormat::DXT1 => TextureFormat::Bc1RgbaUnormSrgb,
        GVFormat::DXT3 => TextureFormat::Bc2RgbaUnormSrgb,
        GVFormat::DXT5 => TextureFormat::Bc3RgbaUnormSrgb,
        GVFormat::BC7 => TextureFormat::Bc7RgbaUnormSrgb,
    }
}

impl<Reader: Read + Seek> CompressedImageDataProvider for GVMoviePlayer<Reader> {
    fn set_compressed_image_data(&mut self, image: &mut Image) {
        let image_data = self.get_image_data();
        image.data = image_data.data;
        image.texture_descriptor.format = image_data.format;
        image.texture_descriptor.size = Extent3d {
            width: image_data.resolution.0,
            height: image_data.resolution.1,
            depth_or_array_layers: 1,
        };
    }

    fn get_compressed_image_data(&mut self) -> ImageData {
        match self.get_state() {
            PlayingState::Stopped => {
                // FIXME: slow? need cached for first and last frame?
                let frame_or_not = if self.get_blank_mode() == BlankMode::LastFrameOnPause_FirstFrameOnStop {
                    self.gv.read_frame_compressed(0).ok()
                } else if self.get_blank_mode() == BlankMode::LastFrameOnPauseAndStop {
                    self.gv.read_frame_compressed(self.gv.get_frame_count() - 1).ok()
                } else {
                    None
                };
                let frame_data = if let Some(frame) = frame_or_not {
                    Some(ImageData {
                        data: frame,
                        format: get_texture_format(self.gv.get_format()),
                        resolution: self.gv.get_resolution(),
                    })
                } else {
                    None
                };
                get_blank_frame_bgra(self.get_blank_mode(), self.get_state(), frame_data)
            }
            PlayingState::Paused => {
                // FIXME: slow? need cached for first and last frame?
                let last_frame = self.gv.read_frame_compressed_at(self.get_position()).ok();
                let last_frame_data = if let Some(frame) = last_frame {
                    Some(ImageData {
                        data: frame,
                        format: get_texture_format(self.gv.get_format()),
                        resolution: self.gv.get_resolution(),
                    })
                } else {
                    None
                };
                get_blank_frame_bgra(self.get_blank_mode(), self.get_state(), last_frame_data)
            }
            PlayingState::Playing => {
                let frame = self.gv.read_frame_compressed_at(self.get_position()).ok();
                let frame_data = if let Some(frame) = frame {
                    ImageData {
                        data: frame,
                        format: get_texture_format(self.gv.get_format()),
                        resolution: self.gv.get_resolution(),
                    }
                } else {
                    // WORKAROUND
                    get_blank_frame_bgra(self.get_blank_mode(), self.get_state(), None)
                };
                frame_data
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
        let mut movie = load_gv("assets/test.gv");
        movie.play();
        movie.pause();
        movie.stop();
    }

    // TODO: add duration test
    // TODO: add loop test
    // TODO: add seek test
    // TODO: add image data test
}
