FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /imageconverter

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /imageconverter/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin imageconverter

FROM alpine AS runtime
EXPOSE 8000

COPY --from=builder /imageconverter/target/x86_64-unknown-linux-musl/release/imageconverter . 

VOLUME ./uploads uploads
VOLUME ./templates templates
COPY --from=builder /imageconverter/templates/homepage.md templates/homepage.md
CMD ["./imageconverter"]
