FROM rustlang/rust:nightly-buster as builder
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR app
COPY . .

RUN cargo build --release

FROM scratch as runtime

COPY --from=builder /app/target/release/rzephir /usr/local/bin

USER 1000
ENTRYPOINT ["./usr/local/bin/rzephir"]
