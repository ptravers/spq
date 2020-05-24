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
	@echo "\tmake test  				  - update dependencies"
	@echo ""

clean:
	cargo clean

fmt:
	cargo fmt

vet:
	cargo clippy

fix: vet
	#	doesn't currently work see
	#	https://github.com/rust-lang/rust-clippy/issues/3837
#	cargo fix -Z unstable-options --clippy

build: fmt fix
	cargo build --release

test: build
	cargo test --verbose

clean-build: clean build

clean-test: clean test

update:
	cargo update
