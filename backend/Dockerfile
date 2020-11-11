FROM ekidd/rust-musl-builder:stable AS builder

COPY Cargo.toml ./
COPY src ./src

RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release && \
    strip /home/rust/src/target/x86_64-unknown-linux-musl/release/leo-backend

FROM scratch

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/leo-backend /

EXPOSE 8088

ENTRYPOINT ["/leo-backend"]
