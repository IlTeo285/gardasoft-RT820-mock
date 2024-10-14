FROM debian:stable-slim

LABEL version="1.0"
LABEL description="Gardasoft RT820 Mock"

COPY target/release/gardasoft-mock-rs /usr/bin/gardasoft-mock-rs
ENV RUST_LOG=trace
EXPOSE 30313
ENTRYPOINT ["/usr/bin/gardasoft-mock-rs"]