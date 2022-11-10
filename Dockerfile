# axum 所需，正常的 rust 无法正常编译
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

ADD --chown=rust:rust . ./

RUN cargo build --release
RUN strip -s /home/rust/src/target/x86_64-unknown-linux-musl/release/goods-server

FROM alpine:latest

# 不使用缓存
RUN apk --no-cache add ca-certificates

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/goods-server \
    /usr/local/bin/

WORKDIR /usr/local/bin/

# 设置工作目录
CMD /usr/local/bin/goods-server