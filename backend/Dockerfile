# Build stage 
ARG REMOTE_DATABASE_PATH="/var/lib/kiggyserve/data.sqlite"
ARG STRIPE_SECRET
ARG STRIPE_KEY
ARG COMPLETION_REDIRECT=kiggyshop.com/completed
FROM rust:latest AS build
WORKDIR /app

ENV STRIPE_SECRET=${STRIPE_SECRET} 
ENV STRIPE_KEY=${STRIPE_KEY}
ENV COMPLETION_REDIRECT=${COMPLETION_REDIRECT} 
ENV REMOTE_DATABASE_PATH=${REMOTE_DATABASE_PATH}

COPY model/ ../model
RUN apt-get upgrade && apt-get install clang pkgconf openssl-dev sqlite sqlite-dev

# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for 
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=backend/src,target=src \
  --mount=type=bind,source=backend/Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=backend/Cargo.lock,target=Cargo.lock \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  <<EOF
set -e
cargo build --release
cp ./target/release/kiggyserve /app/kiggyserve
EOF

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
FROM alpine:latest AS final
WORKDIR /var/lib/kiggyshop

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
# ARG UID=10001
# RUN adduser \
#  --disabled-password \
#  --gecos "" \
#  --home "/nonexistent" \
#  --shell "/sbin/nologin" \
#  --no-create-home \
#  --uid "${UID}" \
#  appuser
# USER appuser
#

RUN apk add clang pkgconf openssl-dev sqlite

COPY ./backend/data.sqlite ${REMOTE_DATABASE_PATH}

# Copy the executable from the "build" stage.
COPY --from=build /app/kiggyserve ./kiggyserve

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["./kiggyserve"]
