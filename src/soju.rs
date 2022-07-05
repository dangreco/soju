use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub trait Soju {
  
  fn new(canvas: HtmlCanvasElement) -> Self;
  
  fn get_ctx(&self) -> CanvasRenderingContext2d;
  
  fn get_size(&self) -> (u32, u32);

  fn render_frame(&self, buffer: &Vec<u8>);

  fn render(&self)
  {
    let ctx = self.get_ctx();
    let (width, height) = self.get_size();

    let pixels = (width * height) as usize;
    let buffer: Vec<u8> = vec![0; pixels * 4];
    
    self.render_frame(&buffer);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), width, height).unwrap();
    ctx.put_image_data(&data, 0., 0.).unwrap();
  }

}