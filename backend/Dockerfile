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
RUN apt-get update && apt-get install -y clang pkg-config libssl-dev libsqlite3-dev

# Ensure source files and dependencies are copied correctly
COPY backend/ .

# Build the project in release mode
RUN cargo build --release

# Verify that the executable was successfully built
RUN ls -l ./target/release/kiggyserve

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.
FROM alpine:latest AS final
WORKDIR /kiggyshop

RUN apk add --no-cache openssl sqlite

# Ensure the data file is in the correct place
COPY ./backend/data.sqlite ${REMOTE_DATABASE_PATH}

# Copy the executable from the "build" stage.
COPY --from=build /app/target/release/kiggyserve ./kiggyserve

# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["./kiggyserve"]
