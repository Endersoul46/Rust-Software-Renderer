use minifb::{Key, Window, WindowOptions};
use glam::*;
use software_renderer::buffer::Buffer;
use software_renderer::rasterizer;
use software_renderer::buffer;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const FPS:usize = 60;

fn main() {
    let delta:f32 = 1.0/60.0;
    let mut buf = buffer::Buffer::new(WIDTH, HEIGHT);
    buf.screen = vec![Vec4::new(0.0, 0.0, 0.0, 0.0); WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut triangle = rasterizer::gen_tri(60);

    window.set_target_fps(FPS);
    let mut red_buf: Vec<u32>  =  vec![0; WIDTH * HEIGHT];

    let prev_buf: Buffer = buf.clone();

   let purp = Vec4::new(0.4, 0.0, 0.6, 1.0);
   let purp_buf = Buffer {screen: vec![purp; WIDTH * HEIGHT], width: WIDTH, height: HEIGHT};
   let mut t:f32 = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {    
        t += 5.0*delta;
        triangle = triangle + Vec2::new(10.0*t.cos() , 10.0*t.sin());
        for y in 0..HEIGHT{
            for x in 0..WIDTH {
                let screen_pos:usize = y*WIDTH + x; 
                if triangle.is_inside(Vec2::new(x as f32, y as f32)) {
                    let r = (x as f32) / (WIDTH as f32 - 1.0);
                    let g = (y as f32) / (HEIGHT as f32 - 1.0);
                    buf.screen[screen_pos] =  Vec4::new(r,g,0.0,0.5);
                }else{
                    buf.screen[screen_pos] = purp;
                } 
            }
        }
        buf.prep_buffer(&mut red_buf, &purp_buf);
        window.update_with_buffer(&red_buf, WIDTH, HEIGHT).unwrap();
    }
}
