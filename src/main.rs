use gen_aoc::cli::Cli;
use gen_aoc::gen::Gen;

fn main() {
    let args = Cli::new();
    Gen::new(args.day(), args.fresh()).generate(args.lang());
}
