.PHONY: help build run test clean docker-up docker-down seed

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project
	cargo build --release

run: ## Run the API server
	cargo run --bin ev-api

test: ## Run tests
	cargo test --workspace

clean: ## Clean build artifacts
	cargo clean

docker-up: ## Start all services with Docker Compose
	docker-compose up -d

docker-down: ## Stop all Docker services
	docker-compose down

docker-build: ## Build Docker image
	docker build -t ev-knowledge-graph-api .

docker-logs: ## View Docker logs
	docker-compose logs -f

seed: ## Seed the database with Indian EV data
	cargo run --bin ev-data -- seed

migrate: ## Run database migrations
	cargo run --bin ev-data -- migrate

dev: ## Start development environment
	docker-compose up -d neo4j redis
	@echo "Waiting for services to be ready..."
	@sleep 5
	cargo run --bin ev-api

fmt: ## Format code
	cargo fmt --all

lint: ## Run clippy
	cargo clippy --workspace --all-targets -- -D warnings

check: fmt lint test ## Run all checks (format, lint, test)
