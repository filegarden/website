# syntax=docker/dockerfile:1

# We use Alpine Linux since it's lightweight (much smaller and faster to build).
# TODO: Compare runtime performance on Alpine Linux vs. the default (Debian),
# since they use different C library implementations.
FROM rust:1.95-alpine AS build

WORKDIR /app

# Install all required build dependencies.
RUN apk add --no-cache clang lld musl-dev git file openssl-dev \
    openssl-libs-static

COPY . .

# Persist directories with downloaded or compiled dependencies between builds so
# every build doesn't have to redownload and recompile all dependencies. Then
# build the binary package in release mode, and copy it out of the cache mount
# before it's unmounted after this step.
RUN --mount=type=cache,target=target \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry \
    <<END
set -eu
cd backend
cargo build --locked --release --package backend
cp ./target/release/backend /bin/app
END

# The final image will be a lean Alpine instance with only the final binary from
# the above build image copied into it.
FROM alpine:3 AS final

# Create a non-privileged user for the app to run under.
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home /nonexistent \
    --shell /sbin/nologin \
    --no-create-home \
    --uid 10001 \
    app

USER app

COPY --from=build /bin/app /bin/app

ENTRYPOINT [ "/bin/app" ]
