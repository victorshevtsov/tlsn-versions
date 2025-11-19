# Version v0.1.0-alpha.13

## CoinGecko example

The CoinGecko API responds with a header `Transfer-Encoding` set to `chunked`. That makes it impossible to notarize the responses because TLSN does not support `Transfer-Encoding`. I tried to downgrade the HTTP version to 1.0 so the server won't send the headers. In turn, it fails with another error message: `A response with a body must contain either a Content-Length or Transfer-Encoding header`

To reproduce the issue run:

```bash
API_KEY=<coingecko_api_key> cargo run --release --example coingecko
```

## Prove example

The example performs a series of proofs of different payload sizes measuring time elapsed

```bash
./test.sh
```

Results:

```
https://mock.verity.usher.so/1kb.json    elapsed 1.52s
https://mock.verity.usher.so/2kb.json    elapsed 1.63s
https://mock.verity.usher.so/4kb.json    elapsed 1.86s
https://mock.verity.usher.so/8kb.json    elapsed 2.59s
https://mock.verity.usher.so/16kb.json   elapsed 5.59s
https://mock.verity.usher.so/32kb.json   elapsed 20.51s
https://mock.verity.usher.so/64kb.json   elapsed 155.21s
https://mock.verity.usher.so/128kb.json  elapsed 1210.78s
```
