PROJECT_NAME := gen-aoc

all: build

build:
	cargo build

build-release:
	cargo build --release

run: build
	./target/debug/$(PROJECT_NAME) 

run-release: build-release
	./target/release/$(PROJECT_NAME)

lint:
	cargo clippy
	cargo fmt

clean:
	cargo clean

.PHONY: all build build-release run run-release lint clean
