use std::sync::{Arc, Mutex};

use crate::soju::Soju;
use rayon::prelude::*;

pub struct Tile {
  x0: u32,
  y0: u32,
  x1: u32,
  y1: u32,
}

pub trait TileRenderer 
where
  Self: Soju + Send + Sync,
{

  fn subdivide(&self) -> Vec<Tile>
  {
    let mut tiles: Vec<Tile> = Vec::new();

    let tile_size: usize = 16;
    let t = tile_size as u32;
    let (width, height) = self.get_size();

    for x0 in (0..width).step_by(tile_size) {

      let x1 = if x0 + t > width { width } else { x0 + t };
      
      for y0 in (0..height).step_by(tile_size) {
      
        let y1 = if y0 + t > height { height } else { y0 + t };

        tiles.push(Tile { x0, y0, x1, y1 });
      }
    }

    tiles
  }

  fn render_tile(&self, tile: &Tile) -> Vec<u8>;

  fn render_frame(&self, buffer: &mut Vec<u8>)
  {
    let tiles = self.subdivide();
    let (width, _) = self.get_size();

    let buf = Arc::new(Mutex::new(buffer));
   
    tiles
      .par_iter()
      .for_each(|tile| {
        let mut b = buf.lock().unwrap();
        let pixels = self.render_tile(tile);

        let x0 = tile.x0;
        let y0 = tile.y0;
        let x1 = tile.x1;
        let y1 = tile.y1;

        let w = x1 - x0;

        for y in y0..y1 {

          let i_b_ = y * width;
          let i_p_ = (y - y0) * w;

          for x in x0..x1 {
            let i_b = (i_b_ + x) as usize;
            let i_p = (i_p_ + (x - x0)) as usize;

            b[i_b] = pixels[i_p];
            b[i_b + 1] = pixels[i_p + 1];
            b[i_b + 2] = pixels[i_p + 2];
            b[i_b + 3] = pixels[i_p + 3];
          }
        }

      });

  }

}