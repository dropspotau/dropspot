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

COPY web/public /app/web/public
COPY web/src /app/web/src
COPY web/build.sh /app/web/build.sh
COPY web/package.json /app/web/package.json
COPY web/pnpm-lock.yaml /app/web/pnpm-lock.yaml
COPY web/pnpm-workspace.yaml /app/web/pnpm-workspace.yaml
COPY web/tsconfig.json /app/web/tsconfig.json
COPY web/vite.config.ts /app/web/vite.config.ts

# Run the server as a custom user so we don't accidentally access any root files
RUN useradd server
RUN chown -R server:server /app
RUN chown -R server:server /home
USER server

# Install pnpm for the web build - https://pnpm.io/installation#in-a-docker-container
ENV PNPM_HOME="/home/server/.local/share/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.bashrc" SHELL="$(which bash)" bash -
RUN chmod +x $PNPM_HOME
# RUN SHELL=$(which bash) ls /home/server/.local/share/pnpm && exit 1
RUN SHELL=$(which bash) pnpm install

ENV SQLX_OFFLINE=true
RUN SHELL=$(which bash) /app/scripts/deploy-build.sh $SHOULD_MIGRATE

CMD ["cargo", "run", "--package", "dropspot-server", "server", "run"]
