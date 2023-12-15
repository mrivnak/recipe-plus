FROM rust:latest as build

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

# Install npm
RUN apt update && apt install -y npm

# Install trunk and diesel
RUN cargo install trunk
RUN cargo install diesel_cli --no-default-features --features sqlite

WORKDIR /app
COPY . .
RUN rm -f db.sqlite

# Run migrations
ENV DATABASE_URL=/app/db.sqlite
RUN cd recipe-plus-server && diesel migration run

# Build backend
RUN cargo build --release --bin recipe-plus-server

# Build frontend
RUN cd recipe-plus && trunk build --release

FROM nginx:mainline

RUN apt update && apt install -y python3

COPY --from=build /app/dist /usr/share/nginx/html
COPY --from=build /app/recipe-plus/nginx.conf /etc/nginx/nginx.conf
COPY --from=build /app/target/release/recipe-plus-server /usr/bin/recipe-plus-server
COPY --from=build /app/recipe-plus-server/db.sqlite3 /app/db.sqlite
COPY --from=build /app/run.py /app/run.py

WORKDIR /app

ARG NGINX_PORT=3000

EXPOSE $NGINX_PORT
EXPOSE 8000

RUN sed -i "s/listen 3000;/listen $NGINX_PORT;/" /etc/nginx/nginx.conf

CMD ["/usr/bin/python3", "run.py", "--prod"]
