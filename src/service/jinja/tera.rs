use super::Jinja;
use ext_tera::{Context, Tera};
use serde::Serialize;
use tera as ext_tera;

pub struct TeraEngine {
    engine: Tera,
}

impl TeraEngine {
    pub fn new(path: &str) -> &Self {
        let t = Tera::new(path).expect("could not initialize template engine");
        &TeraEngine { engine: t }
    }
}

impl Jinja for TeraEngine {
    fn render<T: Serialize>(&self, template: &str, cx: T) -> Result<String, String> {
        let context = Context::from_serialize(cx).unwrap();
        match self.engine.render(template, &context) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::prelude::*;

    #[derive(Serialize)]
    struct Values<'a> {
        you: &'a str,
    }

    #[test]
    fn render_page() {
        let dir = "/tmp/templates/bucket";
        let mut file = fs::create_dir_all(dir)
            .and_then(|_| fs::File::create(format!("{}/hello.html", dir)))
            .unwrap();
        file.write_all(b"Hello, {{ you }}").unwrap();
        file.sync_all().unwrap();

        let jinja = TeraEngine::new("/tmp/templates/**/*");
        let page = jinja.render("bucket/hello.html", Values { you: "World" });
        assert_eq!(page, Ok(String::from("Hello, World")));

        fs::remove_dir_all("/tmp/templates").unwrap();
    }
}
