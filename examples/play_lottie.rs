#[cfg(feature = "lottie")]
use std::{fs::File, io::{BufReader, Cursor}, time::Duration};
#[cfg(feature = "lottie")]
use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension}}};
use bevy_movie_player::lottie::LottieMoviePlayer;
#[cfg(feature = "lottie")]
use bevy_movie_player::{movie_player::{CompressedImageDataProvider, ImageDataProvider, LoopMode}, prelude::*};
#[cfg(feature = "lottie")]
use bevy_movie_player::lottie::load_lottie;

#[cfg(feature = "lottie")]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MoviePlayerPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ImageHandle {
            handle: None,
        })
        .insert_resource(MovieRes {
            last_update_time: None,
            movie_player: None,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(Update, update_fps)
        .run();
}

#[cfg(feature = "lottie")]
#[derive(Resource)]
struct MovieRes {
    last_update_time: Option<Duration>,
    movie_player: Option<LottieMoviePlayer>,
}

#[cfg(feature = "lottie")]
#[derive(Resource)]
struct ImageHandle {
    handle: Option<Handle<Image>>,
}

#[cfg(feature = "lottie")]
#[derive(Component)]
struct FpsText;

#[cfg(feature = "lottie")]
fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut image_handle: ResMut<ImageHandle>,
    mut movie_res: ResMut<MovieRes>,
    // mut asset_server: Res<AssetServer>,
    // time: Res<Time>,
) {
    let movie_player = load_lottie("test_assets/test.json");
    movie_res.movie_player = Some(movie_player);

    let movie_player = movie_res.movie_player.as_mut().unwrap();

    movie_player.set_loop_mode(LoopMode::Loop);
    movie_player.play();

    commands.spawn(Camera2dBundle::default());

    // let image_data = movie_player.get_compressed_image_data();

    // WORKAROUND: to avoid panic: Using pixel_size for compressed textures is invalid
    let image_data = movie_player.get_image_data();

    // println!("Image data: {:?}", image_data);

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

    image_handle.handle = Some(images.add(image));

    // background plane
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        ..default()
    });
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(360.0, 360.0)),
            ..default()
        },
        texture: image_handle.handle.clone().unwrap(),
        ..default()
    });

    // fps text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
        ]),
        FpsText,
    ));
}

#[cfg(feature = "lottie")]
fn update(
    mut images: ResMut<Assets<Image>>,
    image_handle: Res<ImageHandle>,
    mut movie_res: ResMut<MovieRes>,
    time: Res<Time>,
) {
    // skip update to be fps 30 (msec 33)
    if movie_res.last_update_time.is_some() {
        let last_update_time = movie_res.last_update_time.unwrap();
        let time_since_startup = time.elapsed();
        if time_since_startup - last_update_time < Duration::from_millis(33) {
            return;
        }
    }

    let movie_player = movie_res.movie_player.as_mut().unwrap();
    movie_player.update(time.elapsed());

    // get image from handle
    let handle = image_handle.handle.clone().unwrap();
    let image = images.get_mut(handle).unwrap();

    // println!("Update image data with time: {}", time.elapsed_seconds());

    movie_player.set_image_data(image);
    
    movie_res.last_update_time = Some(time.elapsed());
}

#[cfg(feature = "lottie")]
fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}