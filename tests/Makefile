.PHONY: help clean test run

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

test: clean
	docker-compose build test
	docker-compose up --exit-code-from test

run: clean build
	docker-compose spq

build-and-test: clean build test
