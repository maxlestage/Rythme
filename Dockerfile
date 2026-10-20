# The website of Rythme (site/, a Paquets app): its server, and its browser side in
# Rust compiled to WebAssembly (with the interpreter for the playground). Heroku builds
# it with the emk/rust buildpack (RustConfig, Procfile); this image is the other way:
# the container stack (heroku.yml), or any Docker host.
#
#   docker build -t rythme-site . && docker run --rm -p 8080:8080 rythme-site

FROM rust:1-slim-bookworm AS build
# The browser side is compiled to WebAssembly, and its glue written by wasm-bindgen-cli
# at the version of the wasm-bindgen crate in site/Cargo.lock (site/build.sh checks it).
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli --version 0.2.129 --locked
WORKDIR /src
COPY . .
# The browser side: site/public/pkg/.
RUN site/build.sh

# The server (the examples are built into it).
RUN cd site && cargo build --release --bin rythme-site && cp target/release/rythme-site /server

# A small image: the program and its static files.
FROM debian:bookworm-slim
RUN useradd --system --uid 10001 rythme
WORKDIR /app
COPY --from=build /server /usr/local/bin/rythme-site
COPY --from=build /src/site/public ./public
USER rythme
# Heroku sets PORT at startup; 8080 elsewhere.
ENV PORT=8080
EXPOSE 8080
CMD ["rythme-site"]
