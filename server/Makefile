.PHONY: clean compile test verify fmt

clean:
	@echo "Removing created binaries"
	@go clean

fmt:
	@echo "Formatting files"
	@go fmt

verify: fmt
	@echo "Verifying current build"
	@go vet

test: fmt verify
	@echo "Running tests"
	@go test ./...

compile: fmt verify
	@echo "Compiling binary"
	@go build -i

setup:
	@echo "Preparing repository for work"
	@brew install pre-commit

