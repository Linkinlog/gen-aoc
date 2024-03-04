package main

import (
	_ "embed"
	"flag"
	"fmt"
	"strings"
	"time"
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
	flag.IntVar(&part, "p", 1, "part 1 or 2")
	flag.Parse()
	fmt.Printf("Running part %d...\n", part)
	start := time.Now()

	if part == 1 {
		ans := part1(strings.Split(input, "\n"))
		fmt.Println(ans)
	} else {
		ans := part2(strings.Split(input, "\n"))
		fmt.Println(ans)
	}

	fmt.Println("Execution time: ", time.Since(start))
}
