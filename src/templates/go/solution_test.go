package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		input string
		want  int
	}{
		// TODO
	}

	for name, tt := range tests {
		t.Run(name, func(t *testing.T) {
			if got := part1(strings.Split(tt.input, "\n")); got != tt.want {
				t.Errorf("() = %#v, want %#v", got, tt.want)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := map[string]struct {
		input string
		want  int
	}{
		// TODO
	}

	for name, tt := range tests {
		t.Run(name, func(t *testing.T) {
			if got := part2(strings.Split(tt.input, "\n")); got != tt.want {
				t.Errorf("() = %#v, want %#v", got, tt.want)
			}
		})
	}
}
