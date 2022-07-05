use crate::soju::Soju;
use rayon::prelude::*;

pub trait XYRenderer 
where
  Self: Soju + Send + Sync,
{

  fn render_xy(&self, x: u32, y: u32) -> [u8; 4];

  fn render_frame(&self, buffer: &mut Vec<u8>)
  {
    let (width, _) = self.get_size();
    buffer
      .par_chunks_mut(4)
      .enumerate()
      .for_each(|(i, chunk)| {
        let i = i as u32;
        let x = i % width;
        let y = i / width;
        let rgba = self.render_xy(x, y);

        match rgba {
          [r,g,b,a] => {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = a;
          }
        }
      });
  }

}