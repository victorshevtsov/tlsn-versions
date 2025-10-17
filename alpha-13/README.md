# Version v0.1.0-alpha.13

## CoinGecko example

The CoinGecko API responds with a header `Transfer-Encoding` set to `chunked`. That makes it impossible to notarize the responses because TLSN does not support `Transfer-Encoding`. I tried to downgrade the HTTP version to 1.0 so the server won't send the headers. In turn, it fails with another error message: `A response with a body must contain either a Content-Length or Transfer-Encoding header`

To reproduce the issue run:

```bash
API_KEY=<coingecko_api_key> cargo run --release --example coingecko
```
