use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::utils::ConditionalSendFuture;
use derivative::Derivative;
use gv_video::get_bgra_vec_from_frame;
use gv_video::GVVideo;
use gv_video::GVFormat;

use crate::blankable_image_data_provider::BGRAImageFrameProvider;
use crate::blankable_image_data_provider::BlankMode;
use crate::blankable_image_data_provider::Blankable;
use crate::blankable_image_data_provider::CompressedImageFrameProvider;
use crate::movie_player::MoviePlayerStateController;
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
    // blankable_controller: BlankableController,
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

impl<Reader: Read + Seek> Blankable for GVMoviePlayer<Reader> {
    fn get_blank_mode(&self) -> BlankMode {
        self.blank_mode
    }

    fn set_blank_mode(&mut self, blank_mode: BlankMode) {
        self.blank_mode = blank_mode;
    }
}

fn opt_bgra_to_u8(frame_or_not: Option<Vec<u32>>) -> Option<Vec<u8>> {
    if let Some(frame) = frame_or_not {
        Some(get_bgra_vec_from_frame(frame))
    } else {
        None
    }
}

impl<Reader: Read + Seek> BGRAImageFrameProvider for GVMoviePlayer<Reader> {
    fn get_first_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let frame_or_not = self.gv.read_frame(0).ok();
        opt_bgra_to_u8(frame_or_not)
    }

    fn get_last_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let frame_or_not = self.gv.read_frame(self.gv.get_frame_count() - 1).ok();
        opt_bgra_to_u8(frame_or_not)
    }

    fn get_paused_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let frame_or_not = self.gv.read_frame_at(self.get_position()).ok();
        opt_bgra_to_u8(frame_or_not)
    }

    fn get_playing_frame_bgra(&mut self) -> Option<Vec<u8>> {
        let frame_or_not = self.gv.read_frame_at(self.get_position()).ok();
        opt_bgra_to_u8(frame_or_not)
    }

}

fn get_texture_format_from_gv_format(gv_format: GVFormat) -> TextureFormat {
    match gv_format {
        GVFormat::DXT1 => TextureFormat::Bc1RgbaUnormSrgb,
        GVFormat::DXT3 => TextureFormat::Bc2RgbaUnormSrgb,
        GVFormat::DXT5 => TextureFormat::Bc3RgbaUnormSrgb,
        GVFormat::BC7 => TextureFormat::Bc7RgbaUnormSrgb,
    }
}

impl<Reader: Read + Seek> CompressedImageFrameProvider for GVMoviePlayer<Reader> {
    fn get_first_frame_compressed(&mut self) -> Option<Vec<u8>> {
        let frame_or_not = self.gv.read_frame_compressed(0).ok();
        frame_or_not
    }

    fn get_last_frame_compressed(&mut self) -> Option<Vec<u8>> {
        let frame_or_not = self.gv.read_frame_compressed(self.gv.get_frame_count() - 1).ok();
        frame_or_not
    }

    fn get_playing_frame_compressed(&mut self) -> Option<Vec<u8>> {
        let frame_or_not = self.gv.read_frame_compressed_at(self.get_position()).ok();
        frame_or_not
    }

    fn get_paused_frame_compressed(&mut self) -> Option<Vec<u8>> {
        let frame_or_not = self.gv.read_frame_compressed_at(self.get_position()).ok();
        frame_or_not
    }

    fn get_texture_format(&self) -> TextureFormat {
        get_texture_format_from_gv_format(self.gv.get_format())
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
