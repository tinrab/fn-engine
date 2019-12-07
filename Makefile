.PHONY: test
test: ## Run tests
	cargo test --workspace

.PHONY: build
build: ## Run build
	cargo build --workspace

.PHONY: lint
lint: ## Run lints
	cargo clippy --all-targets --all-features -- -D warnings \
	&& cargo fmt --all -- --check

.PHONY: help
help: ## Display this help screen
	grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
