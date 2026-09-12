FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# sqlx's query! macros type-check against this cache instead of a live DB,
# which isn't reachable from Coolify's build step. Generate it with
# `cargo sqlx prepare --workspace -- --no-default-features --features server`
# and commit the resulting .sqlx/ folder whenever a query changes.
ENV SQLX_OFFLINE=true

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Create the final bundle folder. Bundle with release build profile to enable optimizations.
RUN dx bundle --web --release

FROM chef AS runtime
COPY --from=builder /app/target/dx/diet-tracker/release/web/ /usr/local/app

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]