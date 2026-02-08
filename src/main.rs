
#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

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

fn main() {
    println!("Hello, world!");
}
