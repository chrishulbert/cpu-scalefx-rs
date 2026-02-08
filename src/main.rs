
#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

// Make a sample pixel image.
fn sample() -> Image {
    let t = 0; // Transparent.
    let b = 0xff; // Black.
    let y = 0xffff00ff; // Yellow.
    let g = 0x00ff00ff; // Green.
    let l = 0x0000ffff; // bLue.
    let w = 0xffffffff; // White.
    let e = 0x888888ff; // grEy.

    Image {
        width: 12,
        height: 12,
        pixels: vec![
            t,t,t,b,b,b,b,b,b,t,t,t,
            t,t,b,y,g,g,w,g,y,b,t,t,
            t,b,y,y,y,y,g,w,g,y,b,t,
            b,g,e,y,y,y,g,w,g,y,y,b,
            b,y,y,y,y,y,g,w,g,y,y,b,
            b,y,g,y,e,e,e,e,e,e,y,b,
            b,e,g,y,w,l,w,w,l,w,y,b,
            b,b,y,y,w,w,e,e,w,w,y,b,
            b,y,y,e,w,w,w,w,w,w,e,b,
            t,b,e,e,e,w,w,w,w,e,b,t,
            t,t,b,b,e,e,e,e,b,b,t,t,
            t,t,t,t,b,e,e,b,t,t,t,t,
        ],
    }
}

const MAX_COLOUR_DISTANCE: f32 = 765.;

// Determine the human-perceived difference between two colours.
// For humanity's sake, r and g and b are weighted differently.
// https://www.compuphase.com/cmetric.htm
// Returns 0 for same colours; 764.83 for white-black; 765 if any are transparent.
fn colour_distance(a: u32, b: u32) -> f32 {
    let a_r = a >> 24;
    let a_g = (a >> 16) & 0xff;
    let a_b = (a >> 8) & 0xff;
    let a_a = a & 0xff;

    let b_r = b >> 24;
    let b_g = (b >> 16) & 0xff;
    let b_b = (b >> 8) & 0xff;
    let b_a = b & 0xff;

    if a_a < 0x80 || b_a < 0x80 { return MAX_COLOUR_DISTANCE } // Consider transparent very different.

    let r_mean = (a_r + b_r) / 2;
    let r = a_r.abs_diff(b_r);
    let g = a_g.abs_diff(b_g);
    let b = a_b.abs_diff(b_b);

    if r == 0 && g == 0 && b == 0 { return 0. } // Save the conplicated calculation below.

    (((((512 + r_mean)*r*r)>>8) + 4*g*g + (((767-r_mean)*b*b)>>8)) as f32).sqrt()
}

#[derive(Debug, Clone, Copy)]
struct PixelWithDistances {
    pixel: u32,
    up_left: f32, // Colour distance to the pixel to the up-left.
    up: f32,
    up_right: f32,
    right: f32,
}
impl PixelWithDistances {
    fn offscreen() -> Self { // The representation for a transparent offscreen pixel.
        Self {
            pixel: 0,
            up_left: MAX_COLOUR_DISTANCE,
            up: MAX_COLOUR_DISTANCE,
            up_right: MAX_COLOUR_DISTANCE,
            right: MAX_COLOUR_DISTANCE,
        }
    }
}

#[derive(Debug)]
struct ImageWithDistances {
    width: usize,
    height: usize,
    pixels: Vec<PixelWithDistances>,
}

// Calculate the colour distances to neighbours.
// This implements pass 0 here:
// https://github.com/libretro/slang-shaders/blob/master/edge-smoothing/scalefx/shaders/scalefx-pass0.slang
fn calculate_distances(image: &Image) -> ImageWithDistances {
    let pixels_len = image.pixels.len();
    let mut pixels: Vec<PixelWithDistances> = Vec::with_capacity(pixels_len);

    for y in 0..image.height {
        for x in 0..image.width {
            let i = y * image.width + x;

            // Get the neighbouring pixels, returning transparent if they're out of bounds.
            let up_left: u32 = if y==0 || x==0 { 0 } else { image.pixels[i - image.width - 1] };
            let up: u32 = if y==0 { 0 } else { image.pixels[i - image.width] };
            let up_right: u32 = if y==0 || x==image.width-1 { 0 } else { image.pixels[i - image.width + 1] };
            let center = image.pixels[i];
            let right = if x==image.width-1 { 0 } else { image.pixels[i + 1] };

            pixels.push(PixelWithDistances {
                pixel: center,
                up_left: colour_distance(center, up_left),
                up: colour_distance(center, up),
                up_right: colour_distance(center, up_right),
                right: colour_distance(center, right),
            });
        }
    }
    ImageWithDistances {
        width: image.width,
        height: image.height,
        pixels,
    }
}


#[derive(Debug)]
struct PixelWithCornerStrengths {
    pixel: u32,
    up_left: f32, // Corner strength. Called X in the shader.
    up_right: f32, // Y in the shader.
    down_right: f32, // Z in the shader.
    down_left: f32, // W in the shader.
}

#[derive(Debug)]
struct ImageWithCornerStrengths {
    width: usize,
    height: usize,
    pixels: Vec<PixelWithCornerStrengths>,
}

const THRESHOLD: f32 = 0.5; // Min 0.01; max: 1; step: 0.01
const IS_FILTER_AA_ENABLED: bool = true;

// https://github.com/libretro/slang-shaders/blob/master/edge-smoothing/scalefx/shaders/scalefx-pass1.slang
fn corner_strength(d: f32, a_x: f32, a_y: f32, b_x: f32, b_y: f32) -> f32 {
	let diff = a_x - a_y;
	let weight_1 = (THRESHOLD - d).max(0.) / THRESHOLD;
    let is_x_g_y = a_x.min(b_x) + a_x  >  a_y.min(b_y) + a_y;
    let x_g_y_diff = if is_x_g_y { diff } else { -diff };
    let weight_2_raw = (1. - d) + x_g_y_diff;
	let weight_2 = weight_2_raw.clamp(0., 1.);
	if IS_FILTER_AA_ENABLED || 2. * d < a_x + a_y { weight_1 * weight_2 * a_x * a_y } else { 0. }
}

// Calculate all the corner strengths.
// Aka "calculate strength of interpolation candidates" according to the shader comment.
// This implements pass 1 here:
// https://github.com/libretro/slang-shaders/blob/master/edge-smoothing/scalefx/shaders/scalefx-pass1.slang
fn calculate_corner_strengths(image: &ImageWithDistances) -> ImageWithCornerStrengths {
    let pixels_len = image.pixels.len();
    let mut pixels: Vec<PixelWithCornerStrengths> = Vec::with_capacity(pixels_len);
    let offscreen = PixelWithDistances::offscreen();

    for y in 0..image.height {
        for x in 0..image.width {
            let i = y * image.width + x;

            // Get the neighbouring pixels, returning transparent if they're out of bounds.
            let up_left = if y==0 || x==0 { offscreen } else { image.pixels[i - image.width - 1] };
            let up = if y==0 { offscreen } else { image.pixels[i - image.width] };
            let left = if x==0 { offscreen } else { image.pixels[i - 1] };
            let center = image.pixels[i];
            let right = if x==image.width-1 { offscreen } else { image.pixels[i + 1] };
            let down_left = if x==0 || y==image.height-1 { offscreen } else { image.pixels[i + image.height - 1] };
            let down = if y==image.height-1 { offscreen } else { image.pixels[i + image.height] };
            let down_right = if x==image.width-1 || y==image.height-1 { offscreen } else { image.pixels[i + image.height + 1] };

            // Calculate the corner strengths:
            let up_left = corner_strength(left.up_right, left.right, center.up, up_left.right, left.up);
            let up_right = corner_strength(right.up_left, center.right, center.up, up.right, right.up);
            let down_right = corner_strength(down.up_right, center.right, down.up, down.right, down_right.up);
            let down_left = corner_strength(down.up_left, left.right, down.up, down_left.right, down_left.up);

            pixels.push(PixelWithCornerStrengths {
                pixel: center.pixel,
                up_left,
                up_right,
                down_right,
                down_left,
            });
        }
    }
    ImageWithCornerStrengths {
        width: image.width,
        height: image.height,
        pixels,
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", colour_distance(0xffffffff, 0xff));
    println!("{}", colour_distance(0xff0000ff, 0xff0000ff));
    println!("{}", colour_distance(0xff0000ff, 0x00ff00ff));
    println!("{}", colour_distance(0xff0000ff, 0x0000ffff));
    println!("{}", colour_distance(0x00ff00ff, 0xff0000ff));
    println!("{}", colour_distance(0x00ff00ff, 0x00ff00ff));
    println!("{}", colour_distance(0x00ff00ff, 0x0000ffff));
    println!("{}", colour_distance(0x0000ffff, 0xff0000ff));
    println!("{}", colour_distance(0x0000ffff, 0x00ff00ff));
    println!("{}", colour_distance(0x0000ffff, 0x0000ffff));
    println!("{}", colour_distance(0x0000ffff, 0));

    let sample = sample();
    let distances = calculate_distances(&sample);
    let corners = calculate_corner_strengths(&distances);
    println!("{:#?}", corners);
}
