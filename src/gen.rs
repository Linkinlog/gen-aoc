use crate::cli;
use crate::go_gen::GoGen;
use crate::rust_gen::RustGen;

pub struct Gen {
    day: u32,
    fresh: bool,
}

impl Gen {
    pub fn new(day: u32, fresh: bool) -> Self {
        Self { day, fresh }
    }
    pub fn generate(&self, lang: cli::Langs) {
        match lang {
            cli::Langs::Rust => RustGen::new(self.day, self.fresh).generate(),
            cli::Langs::Go => GoGen::new(self.day, self.fresh).generate(),
        }
    }
}
