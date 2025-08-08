# Version v0.1.0-alpha.12

## Execution logs

### Prover (test.sh)

```
URI: /32kb.json
Starting an MPC TLS connection with the server
2025-08-08T10:49:37.609786Z  INFO prover: tlsn_prover: starting MPC-TLS
Got a response from the server: 200 OK
2025-08-08T10:49:38.774519Z  INFO prover: tlsn_prover: finished MPC-TLS
2025-08-08T10:49:41.611058Z  INFO prover:notarize:poll{role=Client}:client_handle_inbound: uid_mux::yamux: remote closed connection
2025-08-08T10:49:41.611094Z  INFO prover:notarize:poll{role=Client}: uid_mux::yamux: connection complete
Notarization complete!
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.

URI: /63kb.json
Starting an MPC TLS connection with the server
2025-08-08T10:49:47.515446Z  INFO prover: tlsn_prover: starting MPC-TLS
Got a response from the server: 200 OK
2025-08-08T10:49:48.857443Z  INFO prover: tlsn_prover: finished MPC-TLS
2025-08-08T10:49:53.288864Z  INFO prover:notarize:poll{role=Client}:client_handle_inbound: uid_mux::yamux: remote closed connection
2025-08-08T10:49:53.288905Z  INFO prover:notarize:poll{role=Client}: uid_mux::yamux: connection complete
Notarization complete!
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.

URI: /64kb.json
Starting an MPC TLS connection with the server
2025-08-08T10:49:59.268600Z  INFO prover: tlsn_prover: starting MPC-TLS
Got a response from the server: 200 OK
2025-08-08T10:50:00.647355Z  INFO prover: tlsn_prover: finished MPC-TLS
2025-08-08T10:50:05.074824Z  INFO prover:prove:poll{role=Client}:client_handle_inbound: uid_mux::yamux: remote closed connection
2025-08-08T10:50:05.074852Z  INFO prover:prove:poll{role=Client}: uid_mux::yamux: connection complete
2025-08-08T10:50:05.074867Z ERROR prover:prove: tlsn_prover: error=prover error: commit error caused by: I/O error: unexpected end of file
2025-08-08T10:50:05.074881Z ERROR prover:notarize: tlsn_prover: error=prover error: commit error caused by: I/O error: unexpected end of file
Error: ProverError { kind: Commit, source: Some(EncodingError(Io(Kind(UnexpectedEof)))) }
```

### Notary (notary.sh)

```
2025-08-08T10:49:24.508128Z DEBUG main ThreadId(01) notary_server: Server config loaded:
host: 0.0.0.0
port: 7047
html_info: |
  <head>
    <meta charset="UTF-8">
    <meta name="author" content="tlsnotary">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
  <body>
    <svg width="86" height="88" viewBox="0 0 86 88" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M25.5484 0.708986C25.5484 0.17436 26.1196 -0.167376 26.5923 0.0844205L33.6891 3.86446C33.9202 3.98756 34.0645 4.22766 34.0645 4.48902V9.44049H37.6129C38.0048 9.44049 38.3226 9.75747 38.3226 10.1485V21.4766L36.1936 20.0606V11.5645H34.0645V80.9919C34.0645 81.1134 34.0332 81.2328 33.9735 81.3388L30.4251 87.6388C30.1539 88.1204 29.459 88.1204 29.1878 87.6388L25.6394 81.3388C25.5797 81.2328 25.5484 81.1134 25.5484 80.9919V0.708986Z" fill="#243F5F"/>
      <path d="M21.2903 25.7246V76.7012H12.7742V34.2207H0V25.7246H21.2903Z" fill="#243F5F"/>
      <path d="M63.871 76.7012H72.3871V34.2207H76.6452V76.7012H85.1613V25.7246H63.871V76.7012Z" fill="#243F5F"/>
      <path d="M38.3226 25.7246H59.6129V34.2207H46.8387V46.9649H59.6129V76.7012H38.3226V68.2051H51.0968V55.4609H38.3226V25.7246Z" fill="#243F5F"/>
    </svg>
    <h1>Notary Server {version}!</h1>
    <ul>
      <li>public key: <pre>{public_key}</pre></li>
      <li>git commit hash: <a href="https://github.com/tlsnotary/tlsn/commit/{git_commit_hash}">{git_commit_hash}</a></li>
      <li><a href="healthcheck">health check</a></li>
      <li><a href="info">info</a></li>
    </ul>
  </body>
concurrency: 32
notarization:
  max_sent_data: 1024
  max_recv_data: 132096
  timeout: 1800
  private_key_path: /notary/notary.key
  signature_algorithm: secp256k1
  allow_extensions: false
tls:
  enabled: false
  private_key_path: null
  certificate_path: null
log:
  level: DEBUG
  filter: null
  format: COMPACT
auth:
  enabled: false

2025-08-08T10:49:24.508192Z DEBUG main ThreadId(01) run_server: notary_server::server: Loading notary server's signing key
2025-08-08T10:49:24.508696Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping TLS setup as it is turned off.
2025-08-08T10:49:24.508713Z DEBUG main ThreadId(01) run_server: notary_server::auth: Skipping authorization as it is turned off.
2025-08-08T10:49:24.508753Z  INFO main ThreadId(01) run_server: notary_server::server: Listening for TCP traffic at 0.0.0.0:7047
2025-08-08T10:49:31.847614Z DEBUG main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:49:31.847700Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:49:31.847818Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:49:31.848123Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:49:31.848227Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="8834159f-2ee1-498b-9768-08b5a7d21c70"
2025-08-08T10:49:31.848246Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="8834159f-2ee1-498b-9768-08b5a7d21c70"
2025-08-08T10:49:31.928361Z DEBUG tokio-runtime-worker ThreadId(32) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:49:37.392380Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:49:37.392443Z  INFO tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:49:37.865193Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:49:37.867312Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.332324Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:49:38.333623Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.340826Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:49:38.341196Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:49:38.343960Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.771406Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:38.771441Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.771462Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:38.771478Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.771532Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:38.771550Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.771703Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:38.771726Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.771785Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:38.771791Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.772111Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:38.772130Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.772386Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:38.772406Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.772462Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:38.772493Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:38.772575Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:49:38.774446Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:49:38.774481Z  INFO tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:49:38.774488Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:49:39.413727Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:49:41.610719Z  INFO tokio-runtime-worker ThreadId(31) verifier:notarize: tlsn_verifier: Sent attestation
2025-08-08T10:49:41.721233Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service::tcp: Successful notarization using tcp! session_id="8834159f-2ee1-498b-9768-08b5a7d21c70" elapsed_time_millis=9873
2025-08-08T10:49:41.738330Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:49:41.738442Z  INFO tokio-runtime-worker ThreadId(31) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:49:41.738534Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:49:41.738908Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received upgrade protocol request
2025-08-08T10:49:41.738974Z DEBUG tokio-runtime-worker ThreadId(32) notary_server::service::tcp: Upgraded to tcp connection session_id="13b7ee65-91f5-40f3-8de7-93184a4b0887"
2025-08-08T10:49:41.738989Z DEBUG tokio-runtime-worker ThreadId(32) notary_server::service: Starting notarization... session_id="13b7ee65-91f5-40f3-8de7-93184a4b0887"
2025-08-08T10:49:41.811408Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:49:47.305798Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:49:47.305842Z  INFO tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:49:47.760699Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:49:47.762597Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.219118Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:49:48.220566Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.226761Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:49:48.227169Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:49:48.229600Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.645192Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.645231Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.645254Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.645269Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.645311Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.645326Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.645503Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.645530Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.645563Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.645580Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.646490Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.646508Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.646538Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.646550Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.646631Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.646651Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.648645Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.648667Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.851761Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.851795Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.851828Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.851843Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.852045Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.852071Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.852215Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.852238Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.854055Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.854075Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.854290Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.854308Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.854490Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.854508Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.854807Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.854824Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.855096Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.855114Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.855119Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:49:48.855133Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:48.855229Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:49:48.857323Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:49:48.857380Z  INFO tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:49:48.857397Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:49:49.491295Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:49:53.288583Z  INFO tokio-runtime-worker ThreadId(02) verifier:notarize: tlsn_verifier: Sent attestation
2025-08-08T10:49:53.395174Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Successful notarization using tcp! session_id="13b7ee65-91f5-40f3-8de7-93184a4b0887" elapsed_time_millis=11656
2025-08-08T10:49:53.438057Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:49:53.438108Z  INFO tokio-runtime-worker ThreadId(02) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:49:53.438181Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:49:53.438471Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received upgrade protocol request
2025-08-08T10:49:53.438531Z DEBUG tokio-runtime-worker ThreadId(32) notary_server::service::tcp: Upgraded to tcp connection session_id="9b8ae805-9c57-4dfc-8759-8202f7068f0b"
2025-08-08T10:49:53.438545Z DEBUG tokio-runtime-worker ThreadId(32) notary_server::service: Starting notarization... session_id="9b8ae805-9c57-4dfc-8759-8202f7068f0b"
2025-08-08T10:49:53.512629Z DEBUG tokio-runtime-worker ThreadId(32) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:49:59.054729Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:49:59.054770Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:49:59.519697Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:49:59.521848Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:59.987207Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:49:59.988654Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:49:59.994703Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:49:59.994991Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:49:59.997454Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.427938Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.427977Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.428035Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.428053Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.428125Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.428153Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.428277Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.428300Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.428459Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.428478Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.428833Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.428853Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.429184Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.429205Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.429233Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.429247Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.429486Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.429496Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.429615Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.429633Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.642667Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.642710Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.642993Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.643016Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.643024Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.643029Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.643057Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.643061Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.643109Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.643135Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.643642Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.643667Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.644511Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.644529Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.644995Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.645012Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.645077Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:50:00.645094Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:50:00.645206Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:50:00.647240Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:50:00.647271Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:50:00.647284Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:50:01.285115Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:50:05.074334Z ERROR tokio-runtime-worker ThreadId(32) verifier:verify: tlsn_verifier: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:50:05.074380Z ERROR tokio-runtime-worker ThreadId(32) verifier:notarize: tlsn_verifier: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:50:05.209533Z ERROR tokio-runtime-worker ThreadId(32) verifier:notarize: tlsn_verifier: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:50:05.209584Z ERROR tokio-runtime-worker ThreadId(32) notary_server::service::tcp: Failed notarization using tcp: Error occurred during notarization: verifier error: commit error caused by: I/O error: frame size too big session_id="9b8ae805-9c57-4dfc-8759-8202f7068f0b" elapsed_time_millis=11771
```
