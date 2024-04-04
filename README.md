# bevy_movie_player

![screenshot](./screenshot.png)

**WARN** Still in hot development. The API may change.

- Currently only support .gv format (see https://github.com/funatsufumiya/rust-gv-video).
    - This makes it possible to play video (with alpha channel) on Bevy.
    - both disk stream and on memory stream are supported.
- Planning to support .mp4 format. (using https://github.com/funatsufumiya/rust-mp4)

## Known issues

- Movie load FPS limitation is needed. (example code has 30fps limitation)
- Currently sound is not supported.