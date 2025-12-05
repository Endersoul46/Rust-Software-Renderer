use glam::{Vec3, Vec4, Vec4Swizzles};  

#[derive(Clone)]
pub struct Buffer {
    pub screen: Vec<Vec4>,
    pub width: usize,
    pub height: usize,
}

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn from_u32_rgb(col: u32) -> Vec3 {
    let col = col.to_be_bytes();
    Vec3::new(col[0] as f32,col[1] as f32,col[2] as f32)
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            screen: vec![Vec4::new(0.0, 0.0, 0.0, 0.0); width * height], 
        }
    }

    pub fn prep_buffer_wo_a(&self, red_buf: &mut [u32])  {
        self.screen.iter().enumerate().for_each(|(i,col)| {
            let r = ( col.x*255.0 ) as u8;
            let g = ( col.y*255.0 ) as u8;
            let b = ( col.z*255.0 ) as u8;
            red_buf[i] = from_u8_rgb(r, g, b);
        });
    }


    pub fn apply_alpha(&mut self){
        for pixel in &mut self.screen {
            pixel.x *= pixel.w;
            pixel.y *= pixel.w;
            pixel.z *= pixel.w;
        }

    }

pub fn prep_buffer(&self, red_buf: &mut [u32], prev_buf: &mut Self) {
    self.screen.iter().zip(prev_buf.screen.iter_mut()).enumerate().for_each(|(i,(src_col, col))| {
        let alpha = src_col.w.clamp(0.0, 1.0);
        if alpha > 0.0 {
            let r_b = src_col.x*alpha + col.x * (1.0 - alpha);
            let g_b = src_col.y*alpha + col.y * (1.0 - alpha);
            let b_b = src_col.z*alpha + col.z * (1.0 - alpha);
            let r = (r_b*255.0).clamp(0.0, 255.0) as u8;
            let g = (g_b*255.0).clamp(0.0, 255.0) as u8;
            let b = (b_b*255.0).clamp(0.0, 255.0) as u8;
            red_buf[i] = from_u8_rgb(r, g, b);
            col.x = r_b;
            col.y = g_b;
            col.z = b_b;
        }else{
           self.prep_buffer_wo_a(red_buf);
        }
    });
}
} 





