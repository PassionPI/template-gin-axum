# 时区, 默认为上海
TIMEZONE?=Asia/Shanghai
# 镜像名称, 默认为 app-ink
IMAGE?=app-ink-axum

# 版本号, 需要设置, 默认为 0. 
VERSION?=0
# 下面为所以来环境的变量, 可以根据需要修改
JWT_SECRET?=JWT_SECRET
REDIS_PASSWORD?=redis
PG_USERNAME?=postgres
PG_PASSWORD?=postgres


.PHONY: dev
dev:
	JWT_SECRET=$(JWT_SECRET) \
	REDIS_URI=redis://192.168.31.88:6379 \
  POSTGRES_URI=postgres://postgres:postgres@192.168.31.88:5432/postgres?sslmode=disable \
	cargo run --target=x86_64-unknown-linux-musl --release

.PHONY: fmt
fmt:
	cargo fmt
	cargo clippy

.PHONY: test
test:
	go test -v -cover -count=1 ./...

.PHONY: lint
lint:
	make fmt
	make test

.PHONY: build
build:
	docker build -t $(IMAGE):$(VERSION) .
	
.PHONY: deploy
deploy:
	make build
	IMAGE=$(IMAGE) \
	VERSION=$(VERSION) \
	TIMEZONE=$(TIMEZONE) \
	JWT_SECRET=$(JWT_SECRET) \
	REDIS_PASSWORD=$(REDIS_PASSWORD) \
	PG_USERNAME=$(PG_USERNAME) \
	PG_PASSWORD=$(PG_PASSWORD) \
	docker stack deploy \
		--compose-file=./docker-stack.yml \
		--prune \
		$(IMAGE)


.PHONY: docker-dep
docker-dep:
	sudo docker build -t template-gin-axum:dep -f conf/ci/dockerfile.dep .

.PHONY: docker-build
docker-build:
	sudo docker build -t template-gin-axum:3 -f conf/ci/dockerfile .

.PHONY: docker-run
docker-run:
	sudo docker run --rm -p 9988:3000 template-gin-axum:1
