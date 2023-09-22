ARG RUST_VERSION=1.71.0
ARG APP_NAME=rusty_llama
ARG NODE_MAJOR=20
ARG MODEL_NAME=llama-2-13b-chat.ggmlv3.q4_K_S.bin

FROM node:${NODE_MAJOR} AS tailwind-build

WORKDIR /app
COPY src src
COPY input.css .
COPY tailwind.config.js .
COPY package.json .
RUN npm install
RUN npx tailwindcss -i ./input.css -o ./output.css

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update
RUN apt-get install -y pkg-config openssl libssl-dev curl

COPY . .
COPY --from=tailwind-build /app/output.css /app/style/output.css
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos
RUN cargo leptos build --release -vv

FROM debian:bullseye-slim AS final
ARG APP_NAME
ARG MODEL_NAME

RUN apt-get update && apt-get install -y openssl

WORKDIR /app
COPY --from=build /app/$MODEL_NAME model
COPY --from=build /app/target/server/release/$APP_NAME server
COPY --from=build /app/target/site target/site

ENV MODEL_PATH=/app/model
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

RUN chown -R appuser:appuser /app
RUN chmod -R 755 /app

USER appuser

EXPOSE 3000

CMD ["/app/server"]
