FROM rust:latest as rust-latest-base-image
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup install nightly
WORKDIR /usr/src/tufa_backend
COPY . .
RUN cargo +nightly build --target=x86_64-unknown-linux-musl --release

FROM alpine:latest
RUN addgroup -g 1000 tufa_backend
RUN adduser -D -s /bin/sh -u 1000 -G tufa_backend tufa_backend
WORKDIR /home/rust_rest/bin/
COPY --from=rust-latest-base-image /usr/src/tufa_backend/target/x86_64-unknown-linux-musl/release/tufa_backend .
RUN ls
RUN chown tufa_backend:tufa_backend tufa_backend
USER tufa_backend
EXPOSE 8080
CMD [ "./tufa_backend" ]