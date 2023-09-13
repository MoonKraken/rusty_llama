ARG RUST_VERSION=1.72.0
ARG APP_NAME=rusty_llama
ARG NODE_MAJOR=20
ARG MODEL_NAME=llama-2-13b-chat.ggmlv3.q4_K_S.bin

################################################################################
# tailwind first
FROM node:${NODE_MAJOR} AS tailwind-build
WORKDIR /app
COPY ./input.css .
COPY ./tailwind.config.js .
COPY ./package.json .
RUN npm install
RUN npx tailwindcss -i ./input.css -o ./output.css

################################################################################
# build step
FROM rust:${RUST_VERSION}-bookworm AS build
WORKDIR /app

RUN apt-get update
RUN apt-get install -y pkg-config openssl libssl-dev curl

COPY . .
# copy the tailwind output from the node container
COPY --from=tailwind-build /app/output.css /app/style/output.css
# add WASM
RUN rustup target add wasm32-unknown-unknown
# install leptos build tool
RUN cargo install cargo-leptos
# Now build the Leptos project
RUN cargo leptos build --release -vv

################################################################################
# final image
FROM debian:bookworm-slim AS final
ARG APP_NAME
ARG MODEL_NAME
#
# install openssl
RUN apt-get update && apt-get install -y openssl

# grab the model
COPY --from=build /app/$MODEL_NAME /app/model

# Copy the executable from the "build" stage.
COPY --from=build /app/target/server/release/$APP_NAME /app/server

# Copy the frontend stuff
COPY --from=build /app/target/site /app/target/site

ENV MODEL_PATH=/app/model
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
# because leptos is configured to look in target/site for the static files
WORKDIR /app
# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["/app/server"]
