.PHONY: build test deploy docker-build docker-up
build:
	cargo build --release --workspace
test:
	cargo test --workspace
docker-build:
	docker-compose build
docker-up:
	docker-compose up -d
deploy: docker-build docker-up
	@echo "✅ Deployment complete"
