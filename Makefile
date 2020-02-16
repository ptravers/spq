.PHONY: build clean test help
.DEFAULT: help

help:
	@echo ""
	@echo "Options:"
	@echo "\tmake clean 				  - remove artifacts and containers"
	@echo "\tmake build 				  - build artifacts and images"
	@echo "\tmake test  				  - runs tests in each subdirectory"
	@echo ""

build:
	$(MAKE) -C queue build
	$(MAKE) -C server build
	$(MAKE) -C tests build

test:
	$(MAKE) -C queue test
	$(MAKE) -C server test
	$(MAKE) -C tests test

clean:
	$(MAKE) -C queue clean
	$(MAKE) -C server clean
	$(MAKE) -C tests clean
