FROM rustlang/rust:nightly-buster as builder

WORKDIR app
COPY . .

RUN cargo build --release

FROM bitnami/minideb:buster as runtime

COPY --from=builder /app/target/release/rzephir /usr/local/bin/

USER 1000
ENTRYPOINT ["/usr/local/bin/rzephir"]
