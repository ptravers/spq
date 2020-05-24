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
	sudo rm -rf /var/lib/spqr/*

build:
	docker-compose build

run-test:
	docker-compose build test
	MARKER=nothing docker-compose up --exit-code-from test
	MARKER=durability docker-compose up --exit-code-from test

test: clean build run-test clean

run: clean build
	docker-compose spq

build-and-test: clean build test
