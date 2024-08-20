.PHONY: docker-dep
docker-dep:
	sudo docker build -t template-gin-axum:dep -f conf/ci/dockerfile.dep .

.PHONY: docker-build
docker-build:
	sudo docker build -t template-gin-axum:1 -f conf/ci/dockerfile .

.PHONY: docker-run
docker-run:
	sudo docker run --rm -p 9988:3000 template-gin-axum:1

.PHONY: dev
dev:
	cargo run --target=x86_64-unknown-linux-musl