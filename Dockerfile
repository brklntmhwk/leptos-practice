############################################################################
# Chef Stage
# Invite a chef to serve meals
############################################################################
# Choose a lighter-weight Docker image for static Rust binaries for x86_64 Linux
FROM clux/muslrust:stable AS chef

# Declare args to use at this stage(* Args are defined in docker-compose.yml as args)
ARG PROJECT_DIR

# Execute commands as root hereafter
USER root

# Install cargo-chef
RUN cargo install --locked cargo-chef

# Move to the project dir
# (This is inherited hereafter in other stages with FROM chef AS *** too)
WORKDIR /$PROJECT_DIR

############################################################################
# Planning Stage
# Make a recipe
############################################################################
FROM chef AS planner

# Declare args to use at this stage
ARG APP_DIR

# Copy the app dir
COPY ./$APP_DIR/ ./$APP_DIR/

# Move to the app dir
WORKDIR $APP_DIR

# Create recipe.json based on the Cargo.toml file in the same dir
RUN cargo chef prepare --recipe-path recipe.json

############################################################################
# Cacher Stage
# Buy ingredients (dep packages) and cook a meal based on the recipe
############################################################################
FROM chef AS cacher

# Declare args to use at this stage
ARG PROJECT_DIR
ARG APP_DIR

# Copy recipe.json from the planning stage
COPY --from=planner /$PROJECT_DIR/$APP_DIR/recipe.json recipe.json

# Cook (build) ingredients (dependencies) based on a recipe (recipe.json)
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

############################################################################
# Builder Stage
# Dish it up beautifully
############################################################################
FROM chef AS builder

# Declare args to use at this stage
ARG PROJECT_DIR
ARG APP_DIR
ARG BIN_TARGET

# Copy dependencies and the project
COPY --from=cacher /$PROJECT_DIR/target/ ./target/
COPY --from=cacher /$PROJECT_DIR/ ./

# Build the designated binary of the project
RUN cargo build --release --target x86_64-unknown-linux-musl --bin $BIN_TARGET

############################################################################
# Runtime Stage
# Serve it to a customer so they can savour it as they like
############################################################################
FROM debian:bullseye-slim AS runtime

# Declare args to use at this stage
ARG USERNAME
ARG USER_ID
ARG USER_GID
ARG PROJECT_DIR
ARG BIN_TARGET

# Copy the project app built in the builder stage
COPY --from=builder /$PROJECT_DIR/target/x86_64-unknown-linux-musl/release/$BIN_TARGET /usr/local/bin/

# Add a non-root user and allow them to become root or postgres
RUN groupadd -g $USER_GID $USERNAME \
    && useradd -s /bin/bash -u $USER_ID -g $USER_GID -m $USERNAME || echo "User already exists." \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root, postgres\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# Install the latest version of packages listed below
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

# Add cargo bin and rustup to the PATH
ENV RUSTUP_HOME="/usr/local/rustup" \
    CARGO_HOME="/usr/local/cargo" \
    PATH="/usr/local/cargo/bin:$PATH"

# Install rustup
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

# Update rustup, add WASM to build targets,
# and install required crates on an as-needed basis
RUN set -x \
    && rustup update \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-leptos sea-orm-cli leptosfmt

# Change the ownership of Cargo registry to the non-root user
RUN chown -R $USERNAME:$USERNAME /usr/local/cargo/registry

# Install standalone TailwindCSS CLI, give a permission to execute it to the non-root user
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    && chmod a+x tailwindcss-linux-x64 \
    && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

# Hereafter the non-root user executes commands
USER $USERNAME

# Execute the bin project
CMD ["/usr/local/bin/backend"]
