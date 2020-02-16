.PHONY: help clean test run

.DEFAULT: help

help:
	@echo ""
	@echo "Options:"
	@echo "\tmake clean 				  - remove artifacts"
	@echo "\tmake build 				  - build release artifact"
	@echo "\tmake fmt 				    - format files"
	@echo "\tmake vet 				    - lint and validate files"
	@echo "\tmake test  				  - run tests"
	@echo ""

clean:
	cargo clean

fmt:
	cargo fmt

vet:
	cargo clippy

fix: vet
	cargo fix -Z unstable-options --clippy

test: clean fmt vet
	cargo test --verbose

build: clean fmt vet
	cargo build --release
