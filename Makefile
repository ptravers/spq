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
	$(MAKE) -C storage build
	$(MAKE) -C error build
	$(MAKE) -C integration_tests build

test:
	$(MAKE) -C queue test
	$(MAKE) -C server test
	$(MAKE) -C storage test
	$(MAKE) -C error test
	$(MAKE) -C integration_tests test

clean:
	$(MAKE) -C queue clean
	$(MAKE) -C server clean
	$(MAKE) -C storage clean
	$(MAKE) -C error clean
	$(MAKE) -C integration_tests clean

update:
	$(MAKE) -C queue update
	$(MAKE) -C server update
	$(MAKE) -C storage update
	$(MAKE) -C error update
