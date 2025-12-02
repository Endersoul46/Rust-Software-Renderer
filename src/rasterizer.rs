use glam::{Vec2};
use std::ops::{Mul, Add};

#[derive(Debug)]
pub struct Vertex
{
   pub a: Vec2,
   pub b: Vec2,
   pub c: Vec2,
}

fn is_on_side(a: Vec2, b: Vec2, p: Vec2) -> bool {
    let ab = b - a;
    let ap = p - a; 
    let dot = ap.dot(ab.perp());

    dot <= 0.0 
}

pub fn is_inside(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> bool {
    let ab = is_on_side(a, b, p);
    let bc = is_on_side(b, c, p);
    let ca = is_on_side(c, a, p);

    ab && bc && ca 
}
    
impl Vertex {
    pub fn is_inside(&self, p: Vec2 ) -> bool {
        is_inside(self.a, self.b, self.c, p)
    }
}


impl Mul<Vec2> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: Vec2) -> Vertex {
        Vertex {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
        }
    }
}


impl Add<Vec2> for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vec2) -> Vertex {
        Vertex {
            a: self.a + rhs,
            b: self.b + rhs,
            c: self.c + rhs,
        }
    }
}

pub fn gen_tri(typ: i32) -> Vertex {
    if typ == 1 {
        return Vertex {
            a: glam::vec2(320.0, 180.0 - 144.0), // top vertex (center - half height)
            b: glam::vec2(320.0 - 288.0, 180.0 + 144.0), // bottom left (center - half width, + half height)
            c: glam::vec2(320.0 + 288.0, 180.0 + 144.0), // bottom right
        };
    }
    Vertex {
        a: glam::vec2(280.0, 105.0), // top vertex
        b: glam::vec2(240.0, 175.0), // bottom left
        c: glam::vec2(320.0, 175.0), // bottom right
    }
}




