# syntax=docker/dockerfile:1

# ---------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------
# trixie e non bookworm: i binari precompilati di `dx` richiedono glibc 2.38+,
# bookworm si ferma alla 2.36. Porta anche il PostgreSQL 17 di sistema, lo
# stesso major del `postgres:17-alpine` usato a runtime.
FROM rust:1-trixie AS builder

# Postgres serve solo a compile-time: le macro `sqlx::query_as!` validano le
# query contro un database reale. Ne avviamo uno effimero dentro l'immagine di
# build, così `docker compose build` non dipende da servizi esterni né da una
# cache offline `.sqlx` da tenere aggiornata a mano.
RUN apt-get update \
    && apt-get install -y --no-install-recommends postgresql \
    && rm -rf /var/lib/apt/lists/*

# `dx` (Dioxus CLI) via binstall: scarica il binario precompilato invece di
# ricompilare la CLI da sorgente. La versione segue quella del crate `dioxus`.
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall dioxus-cli --version 0.7.1 --root /usr/local -y --force

# Target del client wasm: dx lo installerebbe da solo, esplicitarlo rende il
# fallimento (se mai capita) più leggibile.
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

# Usata solo dalle macro sqlx durante la compilazione, non a runtime.
ENV DATABASE_URL=postgres://root:root@localhost:5432/diet_tracker

# Le migration vengono applicate al Postgres effimero con psql (in ordine
# lessicografico, che coincide con l'ordine dei timestamp). Il bundle finisce
# nella cache mount, quindi va copiato in /out dentro lo stesso RUN per
# sopravvivere allo stage successivo; dx nomina il binario come il crate, lo
# rinominiamo in `server` così l'ENTRYPOINT non dipende dal nome del package.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target,sharing=locked \
    set -eux; \
    service postgresql start; \
    su postgres -c "psql -v ON_ERROR_STOP=1 -c \"CREATE ROLE root LOGIN SUPERUSER PASSWORD 'root'\""; \
    su postgres -c "createdb -O root diet_tracker"; \
    for f in migrations/*.up.sql; do \
      su postgres -c "psql -v ON_ERROR_STOP=1 -d diet_tracker -f /app/$f"; \
    done; \
    dx bundle --web --release; \
    service postgresql stop; \
    mkdir -p /out; \
    cp -r target/dx/diet-tracker-dioxus/release/web/. /out/; \
    mv /out/diet-tracker-dioxus /out/server

# ---------------------------------------------------------------------------
# Runtime
# ---------------------------------------------------------------------------
FROM debian:trixie-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Contiene il binario `server` e la cartella `public/` con wasm e asset.
COPY --from=builder /out /usr/local/app

ENV IP=0.0.0.0
ENV PORT=8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT ["/usr/local/app/server"]
