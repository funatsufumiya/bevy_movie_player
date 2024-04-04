use bevy::prelude::*;
pub struct MoviePlayerPlugin;

// fn hello_world() {
//     println!("Hello, world!");
// }

impl Plugin for MoviePlayerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, hello_world);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        App::new()
            .add_plugins(MoviePlayerPlugin)
            .update(); // run once
    }
}