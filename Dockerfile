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
ENV NODE_PATH="$HOME/node"
ENV PNPM_HOME="$HOME/.local/share/pnpm"
ENV PATH="$PNPM_HOME/bin:$PATH"

# Installing Node and pnpm with pre-compiled binaries because it was easier than messing around with installation
# script environment variables

# Install Node
# Download and install nvm:
RUN mkdir -p $NODE_PATH/bin
RUN curl -o- https://nodejs.org/dist/v26.5.0/node-v26.5.0-linux-x64.tar.xz > "$NODE_PATH/node.tar.xz"
RUN tar -xJf "$NODE_PATH/node.tar.xz" -C "$NODE_PATH"
RUN rm "$NODE_PATH/node.tar.xz"
RUN mv "$NODE_PATH/node-v26.5.0-linux-x64" "$NODE_PATH/bin/node"
# RUN cat $NODE_PATH/bin/node && exit 1
ENV PATH="$NODE_PATH/bin:$PATH"
# RUN echo $PATH && exit 1
RUN chown -R server:server /home/server/node
RUN chown server:server /home/server/node/bin/node
# RUN ls -las /home/server/node && exit 1
RUN ls -las /home/server && exit 1
RUN node -v

# Install pnpm
RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.bashrc" SHELL="$(which bash)" PNPM_VERSION="11.9.0" bash -
RUN chmod +x $PNPM_HOME

ENV SQLX_OFFLINE=true
RUN /app/scripts/deploy-build.sh $SHOULD_MIGRATE

CMD ["cargo", "run", "--package", "dropspot-server", "server", "run"]
