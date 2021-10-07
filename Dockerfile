# Build-only image
FROM rust:1.55.0-bullseye AS builder
ARG APKTOOL_VERSION=2.6.0
ARG DEBIAN_FRONTEND=noninteractive

WORKDIR /work

# Download apktool
RUN wget https://raw.githubusercontent.com/iBotPeaches/Apktool/master/scripts/linux/apktool && \
  wget -O apktool.jar https://bitbucket.org/iBotPeaches/apktool/downloads/apktool_${APKTOOL_VERSION}.jar

# Build mcc_patch binary
COPY Cargo.toml .
COPY src ./src
RUN ls -l
RUN cargo build --release

# Ensure we have everything
RUN ls -l apktool apktool.jar
RUN ls -l ./target/release/mcc_patch

# Final image
FROM debian:bullseye
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update && \
  apt install -y zipalign apksigner

# Copy recent apktool from build stage
COPY --from=builder /work/apktool /usr/local/bin/
COPY --from=builder /work/apktool.jar /usr/local/bin/
RUN chmod +x /usr/local/bin/apktool && \
  chmod +x /usr/local/bin/apktool.jar

WORKDIR /app
# Copy mcc_patch binary from build stage
COPY --from=builder /work/target/release/mcc_patch .

# Copy auxiliary data from repo
COPY ./patches.yml .
COPY ./keystore.jks .

# Ensure we have all tools in place
RUN ./mcc_patch -h
RUN apksigner --version
RUN apktool --version
RUN which zipalign