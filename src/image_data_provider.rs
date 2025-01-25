use bevy::{asset::RenderAssetUsages, image::Image, prelude::*, render::render_resource::{Extent3d, TextureDimension}};
use crate::movie_player::ImageData;

pub trait ImageDataProvider {
    /// set image data to image with uncompressed texture format (like BGRA8UnormSrgb)
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
    /// returns image data with uncompressed texture format (like BGRA8UnormSrgb)
    fn get_image_data(&mut self) -> ImageData;
}

pub trait CompressedImageDataProvider {
    /// set image data to image with compressed texture format (like BC7Srgb)
    fn set_compressed_image_data(&mut self, image: &mut Image) {
        let image_data = self.get_compressed_image_data();
        image.data = image_data.data;
        image.texture_descriptor.format = image_data.format;
        image.texture_descriptor.size = Extent3d {
            width: image_data.resolution.0,
            height: image_data.resolution.1,
            depth_or_array_layers: 1,
        };
    }
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
