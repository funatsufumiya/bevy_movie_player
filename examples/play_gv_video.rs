use std::{fs::File, io::{BufReader, Cursor}, time::Duration};

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, render::render_resource::{Extent3d, TextureDimension}};
use bevy_movie_player::{gv::{load_gv, load_gv_on_memory, GVMoviePlayer}, prelude::*};

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
#[derive(Resource)]
struct MovieRes {
    last_update_time: Option<Duration>,
    movie_player: Option<GVMoviePlayer<BufReader<File>>>, // for disk stream
    // movie_player: Option<GVMoviePlayer<Cursor<Vec<u8>>>>, // for on memory
}

#[derive(Resource)]
struct ImageHandle {
    handle: Option<Handle<Image>>,
}

#[derive(Component)]
struct FpsText;

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut image_handle: ResMut<ImageHandle>,
    mut movie_res: ResMut<MovieRes>,
    // mut asset_server: Res<AssetServer>,
    time_res: Res<Time>,
) {
    // WORKAROUND
    let time = time_res.clone();

    let movie_player = load_gv("test_assets/test.gv");
    // let movie_player = load_gv("test_assets/test-10px.gv");
    // let movie_player = load_gv("test_assets/countdown.gv");
    // let movie_player = load_gv("test_assets/alpha-countdown.gv");
    // let movie_player = load_gv("test_assets/alpha-countdown-blue.gv");
    // let movie_player = load_gv("test_assets/alpha-countdown-yellow.gv");
    // let movie_player = load_gv_on_memory("test_assets/alpha-countdown-yellow.gv"); // for on memory
    movie_res.movie_player = Some(movie_player);

    let movie_player = movie_res.movie_player.as_mut().unwrap();

    movie_player.play(true, &time);
    // movie_player.play(false, &time);

    commands.spawn(Camera2dBundle::default());

    // texture from bytes
    let image_data = movie_player.get_image_data(&time);

    let image = Image::new(
        Extent3d {
            width: image_data.get_width(),
            height: image_data.get_height(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image_data.data,
        image_data.format,
        // RenderAssetUsages::RENDER_WORLD, // for bevy 0.13.1
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
            custom_size: Some(Vec2::new(640.0, 360.0)),
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

fn update(
    mut images: ResMut<Assets<Image>>,
    image_handle: ResMut<ImageHandle>,
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

    // WORKAROUND
    let time = time.clone();

    let movie_player = movie_res.movie_player.as_mut().unwrap();
    movie_player.update(&time);

    // get image from handle
    let image = images.get_mut(image_handle.handle.clone().unwrap()).unwrap();

    // println!("Update image data with time: {}", time.elapsed_seconds());

    movie_player.set_image_data(image, &time);
    
    movie_res.last_update_time = Some(time.elapsed());
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}