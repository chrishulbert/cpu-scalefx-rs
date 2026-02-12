# ScaleFX-rs

ScaleFX pixel art upcaler, on the CPU (not a shader!), in Rust.

![Blooguard](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Blooguard.png)

![Blooguard](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Blooguard.big.png)

To use: `cargo run in.png out.png`

Original shader algorithm thanks to Sp00kyFox, 2016.

## Why?

![Council](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Council.png)

![Council](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Council.big.png)

ScaleFX already exists as a far-faster GPU shader, so why this project? Fair question. My reasons:

* As a learning exercise.
* For offline upscaling of assets, as opposed to realtime.
* Looks better than XBRZ!

## How to use this in your code

![RoboRed](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/RoboRed.png)

![RoboRed](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/RoboRed.big.png)

Simply copy `scalefx.rs` into your project, and call `scalefx::scale3x(width, height, pixels)`, where pixels is a slice of u32, containing 0xRRGGBBAA data.

If anybody out there actually uses this, we can have a conversation about uplifting this into a proper crate :) 

## Examples

![SQ3](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/sq3.fixed.png)

![SQ3](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/sq3.fixed.big.png)

![Arachnut](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Arachnut.png)

![Arachnut](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Arachnut.big.png)

![Berkeloid](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Berkeloid.png)

![Berkeloid](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Berkeloid.big.png)

![Bird](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Bird.png)

![Bird](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Bird.big.png)

![Bounder](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Bounder.png)

![Bounder](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Bounder.big.png)

![Cloud](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Cloud.png)

![Cloud](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Cloud.big.png)

![Diving](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Diving.png)

![Diving](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Diving.big.png)

![Dopefish](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Dopefish.png)

![Dopefish](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Dopefish.big.png)

![Fish](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Fish.png)

![Fish](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Fish.big.png)

![Keen](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Keen.png)

![Keen](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Keen.big.png)

![Lick](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Lick.png)

![Lick](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Lick.big.png)

![Mushroom](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Mushroom.png)

![Mushroom](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Mushroom.big.png)

![Princess](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Princess.png)

![Princess](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Princess.big.png)

![Rock](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Rock.png)

![Rock](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Rock.big.png)

![Skypest](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Skypest.png)

![Skypest](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Skypest.big.png)

![Slug](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Slug.png)

![Slug](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Slug.big.png)

![Sprite](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Sprite.png)

![Sprite](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Sprite.big.png)

![Thief](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Thief.png)

![Thief](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Thief.big.png)

![Worm](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Worm.png)

![Worm](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Worm.big.png)

![Wormouth](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Wormouth.png)

![Wormouth](https://github.com/chrishulbert/cpu-scalefx-rs/raw/main/readme/Wormouth.big.png)

## References

* https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/shaders
* http://www.compuphase.com/cmetric.htm
* https://docs.libretro.com/development/shader/slang-shaders/#pragma-parameter
