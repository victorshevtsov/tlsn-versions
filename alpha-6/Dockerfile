FROM rust:1.88

RUN cargo install notary-server \
  --git https://github.com/tlsnotary/tlsn \
  --tag v0.1.0-alpha.6 \
  --root notary

ENV PATH="$PATH:/notary/bin"

COPY ./assets/ /notary/

EXPOSE 7047

CMD [ "notary-server", "--config-file", "/notary/config.yaml" ]
