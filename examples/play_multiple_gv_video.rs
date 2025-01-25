use std::{fs::File, io::{BufReader, Cursor}, time::Duration};

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension}}};
use bevy_movie_player::{gv::{load_gv, load_gv_on_memory, GVMoviePlayer}, movie_player::{BlankMode, Blankable, CompressedImageDataProvider, ImageData, ImageDataProvider, LoopMode, PlayingState}, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MoviePlayerPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ImageHandle {
            handles: Vec::new(),
        })
        .insert_resource(MovieRes {
            last_update_time: None,
            movie_players: Vec::new(),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(Update, key_handler)
        .add_systems(Update, update_fps)
        .run();
}
#[derive(Resource)]
struct MovieRes {
    last_update_time: Option<Duration>,
    movie_players: Vec<GVMoviePlayer<BufReader<File>>>, // for disk stream
    // movie_players: Vec<GVMoviePlayer<Cursor<Vec<u8>>>>, // for on memory
}

#[derive(Resource)]
struct ImageHandle {
    handles: Vec<Handle<Image>>,
}

#[derive(Component)]
struct FpsText;

fn is_asset_ready (
    image_handle_res: Res<ImageHandle>,
) -> bool
{
    image_handle_res.handles.len() > 0
}

fn setup(
    mut commands: Commands,
    mut images_res: ResMut<Assets<Image>>,
    mut image_handle: ResMut<ImageHandle>,
    mut movie_res: ResMut<MovieRes>,
    // mut asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    // let movie_player = load_gv("test_assets/test.gv");
    // let movie_player = load_gv("test_assets/test-10px.gv");
    // let movie_player = load_gv("test_assets/countdown.gv");
    // let movie_player = load_gv("test_assets/alpha-countdown.gv");
    // let movie_player = load_gv("test_assets/alpha-countdown-blue.gv");
    // let movie_player = load_gv("test_assets/alpha-countdown-yellow.gv");

    // for disk stream
    let mut movie_players = Vec::<GVMoviePlayer<BufReader<File>>>::new();

    // for on memory
    // let mut movie_players = Vec::<GVMoviePlayer<Cursor<Vec<u8>>>>::new();

    // for disk stream
    // check if the file exists
    if !std::path::Path::new("test_assets/alpha-countdown.gv").exists() {
        println!("test_assets/alpha-countdown.gv not found");
        println!("using test.gv instead");
        movie_players.push(load_gv("assets/test.gv"));
        movie_players.push(load_gv("assets/test.gv"));
        movie_players.push(load_gv("assets/test.gv"));
        movie_players.push(load_gv("assets/test.gv"));
    } else {
        movie_players.push(load_gv("test_assets/alpha-countdown.gv"));
        movie_players.push(load_gv("test_assets/alpha-countdown-red.gv"));
        movie_players.push(load_gv("test_assets/alpha-countdown-green.gv"));
        movie_players.push(load_gv("test_assets/alpha-countdown-blue.gv"));
        movie_players.push(load_gv("test_assets/alpha-countdown-yellow.gv"));
    }

    // for on memory
    // movie_players.push(load_gv_on_memory("test_assets/alpha-countdown.gv"));
    // movie_players.push(load_gv_on_memory("test_assets/alpha-countdown-red.gv"));
    // movie_players.push(load_gv_on_memory("test_assets/alpha-countdown-green.gv"));
    // movie_players.push(load_gv_on_memory("test_assets/alpha-countdown-blue.gv"));
    // movie_players.push(load_gv_on_memory("test_assets/alpha-countdown-yellow.gv"));
    

    movie_res.movie_players = movie_players;

    let movie_players = &mut movie_res.movie_players;

    // play all movies
    for movie_player in movie_players {
        movie_player.set_loop_mode(LoopMode::Loop);
        movie_player.play();

        // movie_player.set_loop_mode(LoopMode::PauseAtEnd);
        // movie_player.set_blank_mode(BlankMode::Transparent);
        // movie_player.set_blank_mode(BlankMode::LastFrameOnPauseAndStop);
        // movie_player.set_blank_mode(BlankMode::LastFrameOnPause_FirstFrameOnStop);
        // movie_player.set_blank_mode(BlankMode::LastFrameOnPause_TransparentOnStop);
        // movie_player.set_blank_mode(BlankMode::Black);
        // movie_player.play();
    }

    commands.spawn(Camera2d::default());

    // texture from bytes
    let mut image_datas = Vec::<ImageData>::new();
    for movie_player in &mut movie_res.movie_players {
        // image_datas.push(movie_player.get_compressed_image_data());

        // WORKAROUND: to avoid panic: Using pixel_size for compressed textures is invalid
        image_datas.push(movie_player.get_image_data());
    }

    let mut images = Vec::<Image>::new();
    for image_data in image_datas {
        images.push(Image::new(
            Extent3d {
                width: image_data.get_width(),
                height: image_data.get_height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            image_data.data,
            image_data.format,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        ));
    }

    for image in images {
        image_handle.handles.push(images_res.add(image));
    }

    // background plane
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    let mut x = -400.0;
    let mut y = -300.0;
    let mut z = 1.0;
    for handle in &image_handle.handles {
        x += 100.0;
        y += 100.0;
        z += 0.1;
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(640.0, 360.0)),
                image: handle.clone(),
                ..default()
            },
            Transform {
                translation: Vec3::new(x, y, z),
                ..default()
            },
        ));
    }

    // fps text
    commands.spawn((
        Text::new("FPS: "),
        TextFont { font_size: 30.0, ..default() },
    )).with_children(|parent| {
        parent.spawn(
        (
            TextSpan::new("0"),
            TextFont { font_size: 30.0, ..default() },
            FpsText,
        ));
    });
}

fn update(
    mut images_res: ResMut<Assets<Image>>,
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

    let movie_players = &mut movie_res.movie_players;
    let mut i = 0;
    for handle in &image_handle.handles {
        let movie_player = &mut movie_players[i];
        movie_player.update(time.elapsed());
        let image = images_res.get_mut(&handle.clone()).unwrap();
        // movie_player.set_image_data(image);
        movie_player.set_compressed_image_data(image); // faster
        i += 1;
    }

    movie_res.last_update_time = Some(time.elapsed());
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.0 = format!("{value:.2}");
            }
        }
    }
}

fn key_handler(
    mut movie_res: ResMut<MovieRes>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // time: Res<Time>,
) {
    let movie_players = &mut movie_res.movie_players;
    for movie_player in movie_players {
        if keyboard_input.just_pressed(KeyCode::Space) {
            // movie_player.pause(&time);

            // toggle play/pause
            match movie_player.get_state() {
                PlayingState::Playing => movie_player.pause(),
                PlayingState::Paused => movie_player.play(),
                PlayingState::Stopped => movie_player.play(),
            }
        }
        if keyboard_input.just_pressed(KeyCode::Enter) {
            // movie_player.stop(&time);
            
            // toggle stop/play
            match movie_player.get_state() {
                PlayingState::Playing => movie_player.stop(),
                PlayingState::Paused => movie_player.stop(),
                PlayingState::Stopped => movie_player.play(),
            }
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            let pos = movie_player.get_position();
            movie_player.seek(pos + Duration::from_secs_f32(1.0));
        }
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            let pos = movie_player.get_position();
            let to_time =
                if pos.as_secs_f32() > 1.0 {
                    pos - Duration::from_secs_f32(1.0)
                } else {
                    Duration::from_secs_f32(0.0)
                };
            movie_player.seek(to_time);
        }
    }
}