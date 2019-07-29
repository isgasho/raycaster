use std::path::Path;

use futures::Future;
use quicksilver::load_file;

use crate::gfx::framebuffer::{PIXEL_SIZE, RGB};
use image::GenericImageView;

pub const TILE_SIZE: usize = 64;
pub const TILE_COUNT: usize = 8;

pub struct Room {
    width: u32,
    height: u32,
    map: Vec<u8>,
    textures: Vec<u8>,
    textures_width: usize,
}

impl Default for Room {
    fn default() -> Self {
        Room::tiled_map("test.tmx")
    }
}

impl Room {
    pub fn tiled_map(name: &str) -> Self {
        let result = tiled::parse_file(&Path::new(name));
        let tiled_map = result.expect(&format!("Failed to load {}", name));

        let map = tiled_map.layers[0]
            .tiles
            .iter()
            .flat_map(|row| row.iter().map(|tile| *tile as u8))
            .collect();

        let bytes = load_file("wolftextures.png")
            .wait()
            .expect("Failed to load textures");
        let image = image::load_from_memory(&bytes).expect("Failed to load textures");
        let textures_width = image.width() as usize;
        let textures = image.to_rgb().into_raw();

        Room {
            width: tiled_map.width,
            height: tiled_map.height,
            map,
            textures,
            textures_width,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.map[(self.width * y + x) as usize]
    }

    pub fn get_color(&self, tile: u8) -> RGB {
        match tile {
            1 => RGB(255, 0, 255),
            2 => RGB(255, 0, 0),
            4 => RGB(128, 128, 128),
            6 => RGB(192, 192, 192),
            7 => RGB(64, 64, 0),
            _ => RGB(0, 0, 0),
        }
    }

    pub fn get_texture_column(&self, tile: u8, x: usize, column_height: usize) -> Vec<RGB> {
        debug_assert!(
            tile >= 1 && tile as usize <= TILE_COUNT,
            format!("No tile with id {}", tile)
        );
        let mut column = vec![RGB(0, 0, 0); column_height];
        for y in 0..column_height {
            let tile_x = (tile - 1) as usize * TILE_SIZE + x;
            let tile_y = y * TILE_SIZE / column_height;
            let offset = (self.textures_width * tile_y + tile_x) * PIXEL_SIZE;
            column[y] = RGB(
                self.textures[offset],
                self.textures[offset + 1],
                self.textures[offset + 2],
            );
        }
        column
    }
}
