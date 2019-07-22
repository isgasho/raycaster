use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::color::Color;

pub struct Framebuffer {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let red = Color::rgb(255, 0, 0);
        Framebuffer {
            width,
            height,
            data: vec![red; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.data[x + y * self.width] = color;
    }

    pub fn write_ppm<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut output = File::create(path)?;
        write!(output, "P6\n{} {}\n255\n", self.width, self.height)?;
        for color in self.data.iter() {
            let (r, g, b, _) = color.bytes();
            output.write_all(&[r, g, b])?;
        }
        Ok(())
    }
}
