use std::{fs::File, io::{self,BufRead, BufReader}};
use glam::{Vec3A};


#[derive(Debug,Clone )]
pub struct Model {
    indicies: Vec<Vec3A>,
    vertices: Vec<Vec3A>
}

impl Model {
    fn new(vertices: Vec<Vec3A>, indicies: Vec<Vec3A>) -> Self {
        Model { vertices, indicies }
    }

    fn from_obj() -> io::Result<Self>  {
        let file = File::open("/Assets/Untitled.obj");
        let reader = BufReader::new(file?);


        let mut vertices: Vec<Vec3A> = vec![];
        let mut indicies: Vec<Vec3A> = vec![];
        
        for line in reader.lines() {

            let raw_line:String = line?;
            match raw_line.chars().nth(0).expect("error while parsing")  {

                'v' => { 

                    let parsed: Vec<f32> = raw_line[1..]
                        .split_whitespace()
                        .map(|x| x.parse::<f32>().expect("error while parsing vertices"))
                        .collect::<Vec<f32>>();
                    vertices.push(Vec3A::new(parsed[0], parsed[1], parsed[2]));
                },
                'f' => {                    
                    let parsed: Vec<f32> = raw_line[1..]
                        .trim()
                        .split(['/',' '])
                        .map(|x| x.parse::<f32>().expect("error while parsing faces"))
                        .collect::<Vec<f32>>();
                    for _i in 0..parsed.len()/3 {
                        indicies.push(Vec3A::new(parsed[0], parsed[1], parsed[2]));
                    } 
                }
                _ => {}
                 
             }  

        }
        Ok(Model { vertices, indicies  })
    }

}
