
# 1. Build the frontend
FROM node:18 AS frontend-build
WORKDIR /app
COPY docker.env ./.env
COPY portfolio_front/package.json portfolio_front/package-lock.json ./
RUN npm install
COPY portfolio_front/ ./
RUN npm run build
RUN ls -lh /app/dist && file /app/dist/*

# 2. Build the Rust backend
FROM rust:latest AS backend-build
WORKDIR /app
# Install build tools
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY portfolio_back/ ./
# Copy built frontend to backend's static folder
# COPY --from=frontend-build /app/dist ./static  
RUN cargo build --release
RUN ls -lh /app/target/release && file /app/target/release/*


# 3. Final image
FROM debian:bookworm-slim
WORKDIR /app
# Install needed runtime libs
RUN apt-get update && apt-get install -y \
   # libssl3 \
    libpq5 \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY docker.env ./.env
COPY --from=frontend-build /app/dist ./static
RUN ls -lh ./static
COPY --from=backend-build /app/target/release/portfolio_back ./server
EXPOSE 8080
CMD ["./server"]
