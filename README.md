# bevy_movie_player

![screenshot](./screenshot.png)

A movie player plugin for Bevy game engine.

**WARN** Still in development, The API may change in the future.

## Supported movie formats

- `.gv` format (`--features gv`)
    - using [rust-gv-video](https://github.com/funatsufumiya/rust-gv-video).
    - alpha channel support.
    - `.gv` has simple LZ4 compressed + BC1/BC2/BC3/BC7 texture format.
    - both disk stream and on memory stream are supported.
- Lottie (lottie-json `.json`) format (`--features lottie`)
    - using [rlottie-rs](https://github.com/msrd0/rlottie-rs)
    - ( supported from `bevy_movie_player` `0.2.1` or higher )
- Any video format supported by [ffmpeg](https://ffmpeg.org/) (`--features ffmpeg`)
    - ***== WARNING ==***: `ffmpeg-sys-next` needs system `ffmpeg` libraries installed. This can be **hard task** for some platforms. For pure Rust solution, I recommend `--features gv` instead.
    - using [video-rs](https://github.com/oddity-ai/video-rs).
    - Currently has extension limitation for bevy-asset-loader. (Need fix [here](https://github.com/funatsufumiya/bevy_movie_player/blob/bdc479e3ebbcefe78e5896ee4d46f1266a56815d/src/ffmpeg.rs#L121-L123) or give options in the future.)
    - ( supported from `bevy_movie_player` `0.4.2` or higher )

## Version compatibility

| Bevy | bevy_movie_player |
|------|-------------------|
| 0.15 | 0.4               |
| 0.14 | 0.3               |
| 0.13 | 0.2               |
| 0.12 | 0.1               |

## Known issues

- Movie loading FPS limitation is needed. (example code has 60fps limitation by `FixedUpdate`)
- Slower FPS on debug build. Please use `--release` flag to check the performance.
  - or set below in your `Cargo.toml`
    ```
    [profile.dev]
    opt-level = 1

    [profile.dev.package."*"]
    opt-level = 3
    ```

- Compressed Texture cannot be used on first frame, to avoid panic: `Using pixel_size for compressed textures is invalid` (`bevy_render-0.12.1/src/texture/image.rs:785:18`).
- No audio support now.

### --feature ffmpeg

- Converting frame into BGRA without no hardware acceleration.

## Planning

- auto image handling / updating (just play/stop, and eliminate `update()` also?)
- pure rust based other codec support.

## Contribution

- Forks and PRs are always welcome :)
