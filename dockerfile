FROM registry.cn-hangzhou.aliyuncs.com/jrjr/rust:1.80.0-slim AS builder

WORKDIR /app

ARG APP_NAME=template-axum
ARG APP_TARGET=x86_64-unknown-linux-musl

ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

RUN rustup target add $APP_TARGET

# 复制 Cargo.toml 和 Cargo.lock 文件到工作目录, 构建依赖项
COPY Cargo.toml Cargo.lock ./
COPY .cargo .cargo
RUN mkdir src && touch src/main.rs && echo 'fn main() {}' >> src/main.rs
RUN cargo build --release --target=$APP_TARGET

COPY src src
RUN cargo build --release --target=$APP_TARGET
RUN mv target/$APP_TARGET/release/$APP_NAME ./x

FROM registry.cn-hangzhou.aliyuncs.com/jrjr/alpine:3.20

WORKDIR /app

ENV TZ=Asia/Shanghai

COPY --from=builder /app/x .

EXPOSE 3000
CMD ["/app/x"]