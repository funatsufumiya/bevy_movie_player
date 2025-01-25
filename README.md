# bevy_movie_player

![screenshot](./screenshot.png)

A movie player plugin for Bevy game engine.

**WARN** Still in development, The API may change in the future.

## Supported movie formats

- Any video format supported by [ffmpeg](https://ffmpeg.org/) (`--features ffmpeg`)
    - using [video-rs](https://github.com/oddity-ai/video-rs).
    - Currently has extension limitation for bevy-asset-loader. (Need fix or modifying code.)
    - ( supported from `bevy_movie_player` `0.4.2` or higher )
- `.gv` format (`--features gv`)
    - using [rust-gv-video](https://github.com/funatsufumiya/rust-gv-video).
    - alpha channel support.
    - `.gv` has simple LZ4 compressed + BC1/BC2/BC3/BC7 texture format.
    - both disk stream and on memory stream are supported.
- Lottie (lottie-json `.json`) format (`--features lottie`)
    - using [rlottie-rs](https://github.com/msrd0/rlottie-rs)
    - ( supported from `bevy_movie_player` `0.2.1` or higher )

## Version compatibility

| Bevy | bevy_movie_player |
|------|-------------------|
| 0.15 | 0.4               |
| 0.14 | 0.3               |
| 0.13 | 0.2               |
| 0.12 | 0.1               |

## Known issues

- Movie load FPS limitation is needed. (example code has 60fps limitation, but considering to use original thread in module)
- Very slow FPS on debug build. Please use `--release` flag to check the performance.
- Compressed Texture cannot be used on first frame, to avoid panic: `Using pixel_size for compressed textures is invalid` (`bevy_render-0.12.1/src/texture/image.rs:785:18`).
- No audio support now.

## Planning

- auto image handling / updating (just play/stop, and eliminate `update()` also?)

## Contribution

- Forks and PRs are always welcome :)
