FROM ekidd/rust-musl-builder:stable AS builder

RUN cargo install wasm-pack

COPY static/index.html ../static/
COPY Cargo.toml ./
COPY src ./src

RUN sudo chown -R rust:rust /home/rust

RUN wasm-pack build --target web --out-name wasm --out-dir /home/rust/static

FROM nginx:stable-alpine

COPY --from=builder /home/rust/static/ /usr/share/nginx/html/
