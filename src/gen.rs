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
                        fs::remove_dir_all(format!("day{}", self.day)).unwrap();
                        fs::create_dir(format!("day{}", self.day)).unwrap();
                    } else {
                        panic!("Directory already exists, use --fresh to overwrite");
                    }
                } else {
                    panic!("Error creating directory: {}", e);
                }
            }
        }
        fs::write(format!("day{}/input.txt", self.day), "").unwrap();
        fs::write(format!("day{}/go.mod", self.day), Self::GO_MOD_TEMPLATE).unwrap();
        fs::write(format!("day{}/main.go", self.day), Self::GO_MAIN_TEMPLATE).unwrap();
        fs::write(
            format!("day{}/solution.go", self.day),
            Self::SOLUTION_TEMPLATE,
        )
        .unwrap();
        fs::write(
            format!("day{}/solution_test.go", self.day),
            Self::TEST_TEMPLATE,
        )
        .unwrap();
        fs::write(
            format!("day{}/solution_benchmark_test.go", self.day),
            Self::TEST_BENCHMARK_TEMPLATE,
        )
        .unwrap();
        println!("Generated Go for day {}, Happy Hacking!", self.day);
    }

    const GO_MOD_TEMPLATE: &'static str = r#"module AdventOfCode

go 1.22
"#;

    const GO_MAIN_TEMPLATE: &'static str = r#"package main

import (
	_ "embed"
	"flag"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func init() {
	input = strings.TrimRight(input, "\n")
	if len(input) == 0 {
		panic("empty input.txt file")
	}
}

func main() {
	var part int
	flag.IntVar(&part, "part", 1, "part 1 or 2")
	flag.Parse()
	fmt.Printf("Running part %d...\n", part)

	if part == 1 {
		ans := part1(input)
		fmt.Println(ans)
	} else {
		ans := part2(input)
		fmt.Println(ans)
	}
}"#;

    const SOLUTION_TEMPLATE: &'static str = r#"package main

import "strings"

func part1(input string) any {
	for _, line := range strings.Split(input, "\n") {
		_ = line
	}

	return nil
}

func part2(input string) any {
	for _, line := range strings.Split(input, "\n") {
		_ = line
	}

	return nil
}"#;

    const TEST_TEMPLATE: &'static str = r#"package main

import "testing"

func TestPart1(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  any
	}{
        // TODO
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := part1(tt.input); got != tt.want {
				t.Errorf("part1() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  any
	}{
        // TODO
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := part2(tt.input); got != tt.want {
				t.Errorf("part2() = %v, want %v", got, tt.want)
			}
		})
	}
}"#;

    const TEST_BENCHMARK_TEMPLATE: &'static str = r#"package main

import "testing"

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part1(input)
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part2(input)
	}
}"#;
}
