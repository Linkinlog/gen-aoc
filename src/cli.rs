use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Langs {
    Go,
    Rust,
}

#[derive(Parser)]
pub struct Cli {
    /// Day of AoC to get, must be between 1 and 25
    #[arg(short, long, default_value = "1")]
    day: u32,
    /// Language for template
    #[arg(short, long, default_value = "go")]
    lang: Langs,
    /// Sometimes we all need a fresh start, passing this flag will overwrite existing files
    #[arg(short, long, default_value = "false")]
    fresh: bool,
}

impl Cli {
    pub fn new() -> Self {
        let args = Cli::parse();
        match args.day {
            1..=25 => {}
            _ => panic!("Day must be between 1 and 25"),
        }
        Self {
            day: args.day,
            lang: args.lang,
            fresh: args.fresh,
        }
    }
    pub fn day(&self) -> u32 {
        self.day
    }
    pub fn lang(&self) -> Langs {
        self.lang.clone()
    }
    pub fn fresh(&self) -> bool {
        self.fresh
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}
