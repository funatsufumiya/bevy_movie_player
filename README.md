# bevy_movie_player

![screenshot](./screenshot.png)

**WARN** Still in hot development. The API may change.

- Currently only support `.gv` format (see https://github.com/funatsufumiya/rust-gv-video).
    - possible to play video (with alpha channel) on Bevy.
    - `.gv` has simple LZ4 compressed + BC1/BC2/BC3/BC7 texture format.
    - both disk stream and on memory stream are supported.

## Known issues

- Movie load FPS limitation is needed. (example code has 30fps limitation, but should use original thread in module)
- Sound is not supported now.

## Planning

- Support .mp4 format. (using https://github.com/funatsufumiya/rust-mp4)
- Support other formats using `ffmpeg` or `gstreamer` as a feature. (example: 
https://gist.github.com/pkupper/108eb8a712f479ecfdb1eaf9b86cd128 )
