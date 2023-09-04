ARG RUST_VERSION=1.72.0
ARG APP_NAME=rusty_llama
ARG NODE_MAJOR=20

FROM rust:${RUST_VERSION}-bookworm AS planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:${RUST_VERSION}-bookworm AS cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:${RUST_VERSION}-bookworm AS build
ARG APP_NAME
WORKDIR /app

# direct apt-get to the latest version of node, which is needed for tailwind
# this is all from here https://github.com/nodesource/distributions#debian-versions
RUN apt-get update
RUN apt-get install -y ca-certificates curl gnupg
RUN mkdir -p /etc/apt/keyrings
RUN curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
RUN echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_20.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list
RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev nodejs
# RUN apk --no-cache add pkgconfig openssl-dev nodejs npm musl-dev

COPY --from=cacher /app/target /target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo install cargo-leptos


COPY . /app
RUN npm install
RUN npx tailwindcss -i ./input.css -o ./style/output.css
RUN rustup target add wasm32-unknown-unknown
RUN cargo leptos build --release -vv
# RUN cp ./target/server/release/$APP_NAME /bin/server
# RUN cp -r ./target/site /bin/site

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the debian bookworm image as the foundation for running the app.
# By specifying the "slim-bookworm" tag, it will also use whatever happens to be the
# most recent version of that tag when you build your Dockerfile. If
# reproducability is important, consider using a digest
# (e.g., debian@sha256:ac707220fbd7b67fc19b112cee8170b41a9e97f703f588b2cdbbcdcecdd8af57).
FROM rustlang/rust:nightly-bookworm AS final
#
# install openssl
RUN apt-get update && apt-get install -y openssl
# RUN apk --no-cache add openssl
# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# grab the model
COPY --from=build /app/llama-2-13b-chat.ggmlv3.q4_K_S.bin /bin/model
ENV MODEL_PATH=/bin/model
# Copy the executable from the "build" stage.
COPY --from=build /app/target/server/release/$APP_NAME /bin/

# Copy the frontend stuff
COPY --from=build /app/target/site /bin/site

# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["/bin/server"]
