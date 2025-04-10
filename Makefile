NAME := beacon

all:
	cargo build
	cp -f target/debug/$(NAME) .

.PHONY: all
