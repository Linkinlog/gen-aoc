use std::fs;

pub struct GoGen {
    day: u32,
    fresh: bool,
}

impl GoGen {
    pub fn new(day: u32, fresh: bool) -> Self {
        Self { day, fresh }
    }
    pub fn generate(&self) {
        println!("Generating Go for day {}", self.day);
        let err = fs::create_dir(format!("day{}", self.day));
        match err {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    if self.fresh {
                        fs::remove_dir_all(format!("day{}", self.day))
                            .expect("Error removing directory");
                        fs::create_dir(format!("day{}", self.day))
                            .expect("Error creating directory");
                    } else {
                        panic!("Directory already exists, use --fresh to overwrite");
                    }
                } else {
                    panic!("Error creating directory: {}", e);
                }
            }
        }
        let input =
            fs::read_to_string("src/templates/go/input.txt").expect("Error reading input.txt");
        let go_mod = fs::read_to_string("src/templates/go/go.mod").expect("Error reading go.mod");
        let go_main =
            fs::read_to_string("src/templates/go/main.go").expect("Error reading main.go");
        let solution =
            fs::read_to_string("src/templates/go/solution.go").expect("Error reading solution.go");
        let solution_test = fs::read_to_string("src/templates/go/solution_test.go")
            .expect("Error reading solution_test.go");
        let solution_benchmark_test = fs::read_to_string("src/templates/go/solution_benchmark.go")
            .expect("Error reading solution_benchmark.go");

        fs::write(format!("day{}/input.txt", self.day), input).expect("Error writing input.txt");
        fs::write(format!("day{}/go.mod", self.day), go_mod).expect("Error writing go.mod");
        fs::write(format!("day{}/main.go", self.day), go_main).expect("Error writing main.go");
        fs::write(format!("day{}/solution.go", self.day), solution)
            .expect("Error writing solution.go");
        fs::write(format!("day{}/solution_test.go", self.day), solution_test)
            .expect("Error writing solution_test.go");
        fs::write(
            format!("day{}/solution_benchmark_test.go", self.day),
            solution_benchmark_test,
        )
        .expect("Error writing solution_benchmark_test.go");
        println!("Generated Go for day {}, Happy Hacking!", self.day);
    }
}
