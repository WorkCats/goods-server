ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release
RUN strip -s /home/rust/src/target/x86_64-unknown-linux-musl/release/goods-server
# Now, we need to build our _real_ Docker container, copying in `using-sqlx`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/goods-server \
    /usr/local/bin/
CMD /usr/local/bin/goods-server