use glam::*;  

pub struct Buffer {
    pub screen: Vec<Vec4>,
    pub width: usize,
    pub height: usize
}

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

impl Buffer {
    pub fn new(bg: f32, width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            screen: vec![Vec4::new(0.0, 0.0, 0.0, 0.0); width * height] 
        }
    }
    

    pub fn prep_buffer(&self, red_buf: &mut [u32])  {
        self.screen.iter().enumerate().for_each(|(i,col)| {
            let r = ( col.x*255.0 ) as u8;
            let g = ( col.y*255.0 ) as u8;
            let b = ( col.z*255.0 ) as u8;
            red_buf[i] = from_u8_rgb(r, g, b);
        }
        );
    } 
}





