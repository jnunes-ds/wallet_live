FROM rust:1-alpine3.23

WORKDIR /app

# Instalar dependências essenciais (se necessário)
RUN apk add --no-cache \
    pkgconfig \
    openssl-dev \
    musl-dev \
    gcc \
    gcompat \
    libstdc++

# Instalar o cargo-bin como solicitado, e o cargo-watch para live-reload
RUN cargo install cargo-bin cargo-watch

# O código fonte e cache serão montados via volume no docker-compose para dev
CMD ["cargo", "run"]
