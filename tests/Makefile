.PHONY: help build clean test run build-and-test

.DEFAULT: help

export COMMIT_HASH=$(shell git rev-parse HEAD | cut -c-7)

help:
	@echo ""
	@echo "Options:"
	@echo "\tmake clean 				  - remove docker-compose containers"
	@echo "\tmake build 				  - build image"
	@echo "\tmake test  				  - runs tests against local docker-compose stack"
	@echo "\tmake build-and-test  - builds and runs tests against local docker-compose stack"
	@echo "\tmake run   				  - build image and start"
	@echo ""

clean:
	docker-compose down

build:
	docker-compose build

run-test:
	docker-compose build test
	docker-compose up --exit-code-from test

test: clean run-test clean

run: clean build
	docker-compose spq

build-and-test: clean build test
