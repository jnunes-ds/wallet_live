FROM rust:1-alpine3.23

WORKDIR /app

# Instalar dependências essenciais (se necessário)
RUN apk add --no-cache \
    bash \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    musl-dev \
    gcc \
    gcompat \
    libstdc++

# Instalar o cargo-run-bin como solicitado, e o cargo-watch para live-reload
RUN cargo install cargo-run-bin cargo-watch

# O código fonte e cache serão montados via volume no docker-compose para dev
CMD ["sleep", "infinity"]
