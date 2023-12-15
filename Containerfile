FROM rust:latest as build

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

# Install npm
RUN apt update && apt install -y npm

# Install trunk and diesel
RUN cargo install trunk
RUN cargo install diesel_cli --no-default-features --features sqlite

WORKDIR /app
COPY recipe-plus-api recipe-plus-api
RUN rm -f db.sqlite

# Run migrations
ENV DATABASE_URL=/app/db.sqlite
RUN cd recipe-plus-api && diesel migration run

# Build backend
RUN cd recipe-plus-api && cargo build --release

FROM node:latest

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app
COPY recipe-plus-web recipe-plus-web

# Build frontend
RUN cd recipe-plus-web && pnpm install && pnpm run build

RUN apt update && apt install -y python3

COPY --from=build /app/recipe-plus-api/target/release/recipe-plus-api /app/recipe-plus-api
COPY --from=build /app/recipe-plus-api/db.sqlite /app/db.sqlite
COPY run.py /app/run.py

ARG NUXT_PORT=3000
ENV NITRO_PORT=$NUXT_PORT
ENV DATABASE_URL=/app/db.sqlite

EXPOSE $NUXT_PORT
EXPOSE 8000

CMD ["/usr/bin/python3", "run.py", "--prod"]
