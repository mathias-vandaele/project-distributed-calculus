FROM rust:latest AS builder

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk --locked

COPY Cargo.toml Cargo.lock index.html style.css ./
COPY src ./src

RUN trunk build --release --public-url /

FROM nginx:alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 8181

CMD ["nginx", "-g", "daemon off;"]
