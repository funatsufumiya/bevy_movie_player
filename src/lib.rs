use bevy::prelude::*;
pub struct MoviePlayerPlugin;

impl Plugin for MoviePlayerPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
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
