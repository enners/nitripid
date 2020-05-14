use serde::Serialize;

pub mod tera;

pub trait Render: Send + Sync {
    fn render<T>(&self, template: &str, cx: T) -> Result<String, String>
    where
        T: Serialize;
}
