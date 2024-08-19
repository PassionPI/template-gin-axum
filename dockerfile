FROM template-gin-axum:dep AS builder

ARG APP_NAME=template-gin-axum
ARG APP_TARGET=x86_64-unknown-linux-musl

COPY src src
RUN cargo build --release --target=$APP_TARGET
RUN mv target/$APP_TARGET/release/$APP_NAME ./x

FROM registry.cn-hangzhou.aliyuncs.com/jrjr/alpine:3.20

WORKDIR /app

ENV TZ=Asia/Shanghai

COPY --from=builder /app/x .

EXPOSE 3000
CMD ["/app/x"]