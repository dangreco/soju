use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Soju {
  ctx: CanvasRenderingContext2d, 
  width: u32,
  height: u32,
}

#[wasm_bindgen]
impl Soju {
  
  #[wasm_bindgen(constructor)]
  pub fn new(
    canvas: HtmlCanvasElement,
  ) -> Soju
  {
    let ctx = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

    let width = canvas.width();
    let height = canvas.height();

    Soju { ctx, width, height }
  }

  #[wasm_bindgen]
  pub fn render(&mut self)
  {
  
  }

}