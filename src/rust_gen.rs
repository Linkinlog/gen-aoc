use std::fs;

pub struct RustGen {
    day: u32,
    fresh: bool,
}

impl RustGen {
    pub fn new(day: u32, fresh: bool) -> Self {
        Self { day, fresh }
    }
    pub fn generate(&self) {
        println!("Generating Rust for day {}", self.day);
        let err = fs::create_dir(format!("day{}", self.day));
        match err {
            Ok(_) => {
                fs::create_dir(format!("day{}/src", self.day))
                    .expect("Error creating src directory");
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    if self.fresh {
                        fs::remove_dir_all(format!("day{}", self.day))
                            .expect("Error removing directory");
                        fs::create_dir(format!("day{}", self.day))
                            .expect("Error creating directory");
                        fs::create_dir(format!("day{}/src", self.day))
                            .expect("Error creating src directory");
                    } else {
                        panic!("Directory already exists, use --fresh to overwrite");
                    }
                } else {
                    panic!("Error creating directory: {}", e);
                }
            }
        }

        let input = include_str!("templates/rust/input.txt");
        let cargo_toml = include_str!("templates/rust/Cargo.toml");
        let main = include_str!("templates/rust/src/main.rs");

        fs::write(format!("day{}/input.txt", self.day), input).expect("Error writing input.txt");
        fs::write(format!("day{}/Cargo.toml", self.day), cargo_toml)
            .expect("Error writing Cargo.toml");
        fs::write(format!("day{}/src/main.rs", self.day), main).expect("Error writing main.rs");

        println!("Generated Rust for day {}, Happy Hacking!", self.day);
    }
}
