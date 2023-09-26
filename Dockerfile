#########################################################################
# Chef Stage
# Invite a chef to serve meals
#########################################################################
FROM clux/muslrust:stable AS chef

# Execute commands as root hereafter
USER root

# Install cargo-chef
RUN cargo install --locked cargo-chef

# Move to the currently working dir (This is inherited hereafter in other stages with FROM chef AS *** too)
WORKDIR /rental-dvd-postgres-db

#########################################################################
# Planning Stage
# Make a recipe
#########################################################################
FROM chef AS planner
COPY ./rental-dvd/ ./rental-dvd/

# Move to the project dir
WORKDIR rental-dvd

# Create recipe.json based on the Cargo.toml file in the same dir
RUN cargo chef prepare --recipe-path recipe.json

#########################################################################
# Cacher Stage
# Buy ingredients (some dep packages) based on the recipe
#########################################################################
FROM chef AS cacher

# Copy recipe.json from the planning stage
COPY --from=planner /rental-dvd-postgres-db/rental-dvd/recipe.json recipe.json

# Cook (build) ingredients (dependencies) based on a recipe (recipe.json)
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

#########################################################################
# Builder Stage
# Get them to cook a meal based on the recipe
#########################################################################
FROM chef AS builder

COPY --from=cacher /rental-dvd-postgres-db/target/ ./target/
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY --from=cacher /rental-dvd-postgres-db/ ./

# Build the designated binary of the project
RUN cargo build --release --target x86_64-unknown-linux-musl --bin backend

#########################################################################
# Runtime Stage
# Serve a meal to a customer so they can savour it as they like
#########################################################################
FROM debian:bullseye-slim AS runtime

# Set non-root user's data (USERNAME should be the same as remoteUser in .devcontainer.json and app.user in docker-compose.yml)
ARG USERNAME=vscode
ARG USER_ID=10001
ARG USER_GID=$USER_ID

# Add a non-root user
RUN groupadd -g $USER_GID $USERNAME \
    && useradd -s /bin/bash -u $USER_ID -g $USER_GID -m $USERNAME -r || echo "User already exists." \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# Automatically install the latest version of packages available listed below
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        git \
        libssl-dev \
        pkg-config \
        postgresql \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the project app built in the builder stage
COPY --from=builder /rental-dvd-postgres-db/target/x86_64-unknown-linux-musl/release/backend /usr/local/bin/

# Make dirs for cargo and rustup
RUN mkdir /usr/local/{rustup,cargo}

# Add cargo bin and rustup to the PATH, and designate the dirs made above as locations for cargo and rustup
ENV RUSTUP_HOME="/usr/local/rustup" \
    CARGO_HOME="/usr/local/cargo" \
    PATH="/usr/local/cargo/bin:$PATH"

# Install rustup
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

# Update rustup, add WASM target to use WebAssembly in a Rust project, and install required crates on an as-needed basis
RUN set -x \
    && rustup update \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-leptos

# Change the ownership of Cargo registry to the non-root user
RUN chown -R $USERNAME /usr/local/cargo/registry

# Hereafter the non-root user executes commands
USER $USERNAME

# Execute the bin project
CMD ["/usr/local/bin/backend"]
