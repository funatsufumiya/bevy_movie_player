# bevy_movie_player

![screenshot](./screenshot.png)

**WARN** Still in development, The API may change in the future.

- Currently only support `.gv` format (see https://github.com/funatsufumiya/rust-gv-video).
    - possible to play video (with alpha channel) on Bevy.
    - `.gv` has simple LZ4 compressed + BC1/BC2/BC3/BC7 texture format.
    - both disk stream and on memory stream are supported.

## Known issues

- Movie load FPS limitation is needed. (example code has 30fps limitation, but considering to use original thread in module)
- Very slow FPS on debug build. Please use `--release` flag to check the performance.
- Sound is not supported now.

## Planning

- Support .mp4 format. (using https://github.com/funatsufumiya/rust-mp4)
- Support other formats using `ffmpeg` or `gstreamer` as a feature. (example: 
https://gist.github.com/pkupper/108eb8a712f479ecfdb1eaf9b86cd128 )
