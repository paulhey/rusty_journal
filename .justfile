_default:
  @just --list

clean:
	cargo clean

build:
	cargo build --locked

install:
	cargo install --path . --locked

update:
	cargo update

cache:
	cargo cache -a
