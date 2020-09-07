.PHONY: build clean test help fmt vet fix update
.DEFAULT: help

help:
	@echo ""
	@echo "Options:"
	@echo "\tmake clean 				  - remove artifacts"
	@echo "\tmake build 				  - build release artifact"
	@echo "\tmake fmt 				    - format files"
	@echo "\tmake vet 				    - lint and validate files"
	@echo "\tmake unit-test  			- run tests"
	@echo "\tmake test  				  - run unit and integration tests"
	@echo "\tmake update  				- update dependencies"
	@echo ""


fmt:
	cargo fmt -- --check

vet:
	cargo clippy

build: fmt vet
	cargo build

build-release: fmt vet
	cargo build --release

unit-test: build
	cargo test --all

integration-test:
	$(MAKE) -C integration_tests build
	$(MAKE) -C integration_tests test

test: unit-test integration-test

clean:
	cargo clean
	$(MAKE) -C integration_tests clean

update:
	cargo update
