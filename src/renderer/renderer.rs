use web_sys::ImageData;

pub trait Renderer: Send + Sync {

  fn render(&self) -> ImageData;

}