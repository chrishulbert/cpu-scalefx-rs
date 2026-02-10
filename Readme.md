# ScaleFX-rs

ScaleFX pixel art upcaler, for the CPU, in Rust.

![Sample](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/sample.png)

![Big](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/out.big.png)

![Bigger](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/out.bigger.png)

## Why?

ScaleFX already exists as a far-faster GPU shader, so why this project? Fair question. My reasons:

* As a learning exercise.
* For offline upscaling of assets, as opposed to realtime.
* Looks better than XBRZ!

## How to use this

Simply copy `scalefx.rs` into your project, and call `scalefx::scale3x(width, height, pixels)`, where pixels is a slice of u32, containing 0xRRGGBBAA data.

If anybody out there actually uses this, we can have a conversation about uplifting this into a proper crate :) 

## References

* https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/shaders
* http://www.compuphase.com/cmetric.htm
* https://docs.libretro.com/development/shader/slang-shaders/#pragma-parameter
