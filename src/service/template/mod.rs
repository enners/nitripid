use serde::Serialize;

pub mod tera;

pub trait Renderer: Send + Sync + Clone {
    fn render<T>(&self, template: &str, cx: T) -> Result<String, String>
    where
        T: Serialize;
}
