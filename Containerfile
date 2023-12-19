FROM rust:latest as build

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

# Install npm
RUN apt update && apt install -y npm

# Install trunk and diesel
RUN cargo install trunk
RUN cargo install diesel_cli --no-default-features --features sqlite

WORKDIR /app
COPY api api
RUN rm -f db.sqlite

# Run migrations
ENV DATABASE_URL=/app/db.sqlite
RUN cd api && diesel migration run

# Build backend
RUN cd api && cargo build --release

FROM node:latest

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app
COPY web web

# Build frontend
RUN cd web && pnpm install && pnpm run build

RUN apt update && apt install -y python3

COPY --from=build /app/api/target/release/recipe-plus /app/recipe-plus
COPY --from=build /app/api/db.sqlite /app/db.sqlite
COPY run.py /app/run.py

ARG NUXT_PORT=3000
ENV NITRO_PORT=$NUXT_PORT
ENV DATABASE_URL=/app/db.sqlite

EXPOSE $NUXT_PORT
EXPOSE 8000

CMD ["/usr/bin/python3", "run.py", "--prod"]
