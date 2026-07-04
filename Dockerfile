FROM rust:1.96 AS builder

ARG DROPSPOT_ENDPOINT
ARG DROPSPOT_DATABASE_URL
ARG DROPSPOT_PORT
ARG SHOULD_MIGRATE

WORKDIR /app
COPY . /app

RUN /app/scripts/deploy-build.sh $SHOULD_MIGRATE

CMD ["cargo", "run", "--package", "dropspot-server", "server", "run"]
