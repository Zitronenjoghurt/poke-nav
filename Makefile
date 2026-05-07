.PHONY: dev-web up down build logs

dev-web:
	cargo install trunk
	cd app && trunk serve --open

dev-nat:
	cargo install cargo-watch
	cargo watch -x "run -p poke-nav-app"

native:
	cargo run --release -p poke-nav-app

up:
	docker compose -f server/docker/docker-compose.yml up -d

down:
	docker compose -f server/docker/docker-compose.yml down

build:
	docker image prune -f
	docker compose -f server/docker/docker-compose.yml build

logs:
	docker compose -f server/docker/docker-compose.yml logs -f