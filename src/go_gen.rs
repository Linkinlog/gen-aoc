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

        let input = include_str!("templates/go/input.txt");
        let go_mod = include_str!("templates/go/go.mod");
        let go_main = include_str!("templates/go/main.go");
        let solution = include_str!("templates/go/solution.go");
        let solution_test = include_str!("templates/go/solution_test.go");
        let solution_benchmark_test = include_str!("templates/go/solution_benchmark.go");

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
