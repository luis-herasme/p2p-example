FROM rust:latest AS builder

RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk

WORKDIR /app

COPY . .

RUN trunk build

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
