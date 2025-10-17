FROM rust:1.88

RUN cargo install notary-server \
  --git https://github.com/tlsnotary/tlsn \
  --tag v0.1.0-alpha.12 \
  --locked

ENV PATH="$PATH:/notary/bin"

COPY ./assets/ /notary/

EXPOSE 7047

CMD [ "notary-server", "--config", "/notary/config.yaml" ]
