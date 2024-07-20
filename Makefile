.PHONY: build
build:
	docker-compose build

.PHONY: run
run: build
	docker-compose up

.PHONY: rust
rust:
	cd ws-server-rs && cargo run

.PHONY: go
go:
	cd ws-server-go && go mod download && go run main.go
