use gen_aoc::cli::Cli;
use gen_aoc::gen::GoGen;

fn main() {
    let args = Cli::new();
    match args.lang() {
        gen_aoc::cli::Langs::Go => {
            let gen = GoGen::new(args.day(), args.fresh());
            gen.generate();
        }
    }
}
