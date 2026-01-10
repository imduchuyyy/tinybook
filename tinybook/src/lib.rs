pub(crate) struct TinyBook {
    title: String,
}

impl TinyBook {
    pub fn new(title: &str) -> Self {
        TinyBook {
            title: title.to_string(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
