# 1. Build the frontend
FROM node:18 AS frontend-build

WORKDIR /app/frontend

COPY portfolio_front/package.json portfolio_front/package-lock.json ./
RUN npm install

COPY portfolio_front/ ./
RUN npm run build


# 2. Build the Rust backend
FROM rust:1.87 AS backend-build

WORKDIR /app

# Install build tools
RUN apt-get update && apt-get install -y pkg-config libssl-dev

COPY portfolio_back/ ./
# Copy built frontend to backend's static folder
COPY --from=frontend-build /app/frontend/dist ./static  

RUN cargo build --release


# 3. Final image
FROM debian:bullseye-slim

WORKDIR /app

# Install needed runtime libs
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=backend-build /app/target/release/portfolio_back ./server
COPY --from=backend-build /app/static ./static

EXPOSE 8080

CMD ["./server"]
