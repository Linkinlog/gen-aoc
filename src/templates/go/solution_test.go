package main

import "testing"

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		input string
		want  any
	}{
        // TODO
	}

	for name, tt := range tests {
		t.Run(name, func(t *testing.T) {
			if got := part1(tt.input); got != tt.want {
				t.Errorf("part1() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := map[string]struct {
		input string
		want  any
	}{
        // TODO
	}

	for name, tt := range tests {
		t.Run(name, func(t *testing.T) {
			if got := part2(tt.input); got != tt.want {
				t.Errorf("part2() = %v, want %v", got, tt.want)
			}
		})
	}
}