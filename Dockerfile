FROM rust:1.96 AS builder

ARG DROPSPOT_ENDPOINT
ARG DROPSPOT_DATABASE_URL
ARG DROPSPOT_PORT
ARG SHOULD_MIGRATE

WORKDIR /app

COPY .sqlx /app/.sqlx
COPY core /app/core
COPY migrations /app/migrations
COPY scripts /app/scripts
COPY server /app/server
COPY static /app/static
COPY Cargo.lock /app/Cargo.lock
COPY Cargo.toml /app/Cargo.toml
COPY rust-toolchain.toml /app/rust-toolchain.toml
COPY sqlx.toml /app/sqlx.toml

# Add web assets, this might eventually become its own Docker image for web-less server images
COPY web/public /app/web/public
COPY web/src /app/web/src
COPY web/build.sh /app/web/build.sh
COPY web/package.json /app/web/package.json
COPY web/pnpm-lock.yaml /app/web/pnpm-lock.yaml
COPY web/pnpm-workspace.yaml /app/web/pnpm-workspace.yaml
COPY web/tsconfig.json /app/web/tsconfig.json
COPY web/vite.config.ts /app/web/vite.config.ts

# Run the server as a custom user so we don't accidentally access any root files
RUN useradd --create-home server
RUN chown -R server:server /app
RUN chown -R server:server /home/server
USER server

# Install pnpm for the web build - https://pnpm.io/installation#in-a-docker-container
ENV HOME="/home/server"
ENV PNPM_HOME="$HOME/.local/share/pnpm"
ENV PATH="$PNPM_HOME/bin:$PATH"

# Installing Node and pnpm with pre-compiled binaries because it was easier than messing around with installation
# script environment variables

# Install Node
# Download and install nvm:
ENV NODE_PATH="node-v26.5.0-linux-x64"
RUN curl -o- https://nodejs.org/dist/v26.5.0/node-v26.5.0-linux-x64.tar.xz > "$NODE_PATH.tar.xz"
RUN tar -xvf "$NODE_PATH.tar.xz"
ENV PATH="$NODE_PATH/bin:$PATH"
# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.5/install.sh | bash
# # in lieu of restarting the shell
# RUN \. "$HOME/.nvm/nvm.sh"
# # Download and install Node.js:
# RUN nvm install 26
# # Verify the Node.js version:
# RUN node -v # Should print "v26.5.0".
# # Install Corepack:
# RUN npm install -g corepack
# # Download and install pnpm:
# RUN corepack enable pnpm
# # Verify pnpm version:
# RUN pnpm -v

# Install pnpm
RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.bashrc" SHELL="$(which bash)" PNPM_VERSION="11.9.0" bash -
RUN chmod +x $PNPM_HOME

ENV SQLX_OFFLINE=true
RUN /app/scripts/deploy-build.sh $SHOULD_MIGRATE

CMD ["cargo", "run", "--package", "dropspot-server", "server", "run"]
