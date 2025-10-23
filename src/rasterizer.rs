use glam::*;

#[derive(Debug)]
pub struct Triangle
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

impl Triangle {
    pub fn is_inside(&self, p: Vec2) -> bool {
        let ab = is_on_side(self.a, self.b, p);
        let bc = is_on_side(self.b, self.c, p);
        let ca = is_on_side(self.c, self.a, p);

        ab && bc && ca 
    }
    
}

use std::ops::Mul;

impl Mul<Vec2> for Triangle {
    type Output = Triangle;

    fn mul(self, rhs: Vec2) -> Triangle {
        Triangle {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
        }
    }
}

use std::ops::Add;
impl Add<Vec2> for Triangle {
    type Output = Triangle;

    fn add(self, rhs: Vec2) -> Triangle {
        Triangle {
            a: self.a + rhs,
            b: self.b + rhs,
            c: self.c + rhs,
        }
    }
}

pub fn gen_tri(typ: i32) -> Triangle {
    if typ == 1 {
        return Triangle {
            a: glam::vec2(320.0, 180.0 - 144.0), // top vertex (center - half height)
            b: glam::vec2(320.0 - 288.0, 180.0 + 144.0), // bottom left (center - half width, + half height)
            c: glam::vec2(320.0 + 288.0, 180.0 + 144.0), // bottom right
        };
    }
    Triangle {
        a: glam::vec2(280.0, 105.0), // top vertex
        b: glam::vec2(240.0, 175.0), // bottom left
        c: glam::vec2(320.0, 175.0), // bottom right
    }
}




