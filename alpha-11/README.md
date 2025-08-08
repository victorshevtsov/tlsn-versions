# Version v0.1.0-alpha.11

## Execution logs

### Prover (test.sh)

```
URI: /32kb.json
Starting an MPC TLS connection with the server
2025-08-08T10:46:22.758315Z  WARN prover: mpc_tls::leader: record layer is not ready, skipping flush
Got a response from the server: 200 OK
2025-08-08T10:46:27.526644Z  INFO prover:notarize:poll{role=Client}:client_handle_inbound: uid_mux::yamux: remote closed connection
2025-08-08T10:46:27.526662Z  INFO prover:notarize:poll{role=Client}: uid_mux::yamux: connection complete
Notarization complete!
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.

URI: /63kb.json
Starting an MPC TLS connection with the server
Got a response from the server: 200 OK
2025-08-08T10:46:40.403132Z  INFO prover:notarize:poll{role=Client}:client_handle_inbound: uid_mux::yamux: remote closed connection
2025-08-08T10:46:40.403167Z  INFO prover:notarize:poll{role=Client}: uid_mux::yamux: connection complete
Notarization complete!
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.

URI: /64kb.json
Starting an MPC TLS connection with the server
Got a response from the server: 200 OK
2025-08-08T10:46:52.891263Z  INFO prover:prove:poll{role=Client}:client_handle_inbound: uid_mux::yamux: remote closed connection
2025-08-08T10:46:52.891299Z  INFO prover:prove:poll{role=Client}: uid_mux::yamux: connection complete
2025-08-08T10:46:52.891328Z ERROR prover:prove: tlsn_prover: error=prover error: commit error caused by: I/O error: unexpected end of file
2025-08-08T10:46:52.891364Z ERROR prover:notarize: tlsn_prover: error=prover error: commit error caused by: I/O error: unexpected end of file
Error: ProverError { kind: Commit, source: Some(EncodingError(Io(Kind(UnexpectedEof)))) }
```

### Notary (notary.sh)

```
2025-08-08T10:46:07.376536Z DEBUG main ThreadId(01) notary_server: Server config loaded:
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
  whitelist_path: null

2025-08-08T10:46:07.376599Z DEBUG main ThreadId(01) run_server: notary_server::server: Loading notary server's signing key
2025-08-08T10:46:07.377162Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping TLS setup as it is turned off.
2025-08-08T10:46:07.377181Z DEBUG main ThreadId(01) run_server: notary_server::auth: Skipping authorization as it is turned off.
2025-08-08T10:46:07.377200Z  INFO main ThreadId(01) run_server: notary_server::server: Listening for TCP traffic at 0.0.0.0:7047
2025-08-08T10:46:16.417528Z DEBUG main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:46:16.417601Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:46:16.417697Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:46:16.418002Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:46:16.418093Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="46c5bf00-5942-4dde-a3e9-28d0c5d9642a"
2025-08-08T10:46:16.418123Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="46c5bf00-5942-4dde-a3e9-28d0c5d9642a"
2025-08-08T10:46:16.519469Z DEBUG tokio-runtime-worker ThreadId(33) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:46:22.262229Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:46:22.262273Z  INFO tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:46:23.037060Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:46:23.202327Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:23.627543Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:46:23.710435Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:23.873967Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:46:23.873995Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:46:23.956383Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467071Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467101Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467120Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467128Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467133Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467139Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467143Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467148Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467154Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467159Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467176Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467191Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467205Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467221Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467265Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467281Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467287Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:24.467304Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:24.467320Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:46:24.757113Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:46:24.757156Z  INFO tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:46:24.771885Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:46:27.106539Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:46:27.526327Z  INFO tokio-runtime-worker ThreadId(33) verifier:notarize: tlsn_verifier: Sent attestation
2025-08-08T10:46:27.645985Z  INFO tokio-runtime-worker ThreadId(33) notary_server::service::tcp: Successful notarization using tcp! session_id="46c5bf00-5942-4dde-a3e9-28d0c5d9642a" elapsed_time_millis=11227
2025-08-08T10:46:27.663883Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:46:27.663963Z  INFO tokio-runtime-worker ThreadId(33) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:46:27.664054Z  INFO tokio-runtime-worker ThreadId(33) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:46:27.664392Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:46:27.664452Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="eb313aa0-dcde-4f0d-851b-33b7859ebb3a"
2025-08-08T10:46:27.664467Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="eb313aa0-dcde-4f0d-851b-33b7859ebb3a"
2025-08-08T10:46:27.757323Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:46:33.694538Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:46:33.694568Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:46:34.390062Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:46:34.554094Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:34.968872Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:46:35.052168Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.216064Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:46:35.216101Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:46:35.298642Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798017Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798050Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798068Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798084Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798100Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798106Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798112Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798117Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798122Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798136Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798143Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798158Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798192Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798206Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798212Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798227Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798233Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798240Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798244Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798251Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.798257Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.798262Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.965892Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.965932Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.965982Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.966001Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.966465Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.966489Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.966519Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.966537Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.966657Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.966676Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.966822Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.966838Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.967019Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.967028Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.967236Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.967256Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.967645Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.967664Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.967704Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:35.967718Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:35.967911Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:46:36.258102Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:46:36.258156Z  INFO tokio-runtime-worker ThreadId(33) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:46:36.286678Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:46:40.004869Z DEBUG tokio-runtime-worker ThreadId(33) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:46:40.402791Z  INFO tokio-runtime-worker ThreadId(02) verifier:notarize: tlsn_verifier: Sent attestation
2025-08-08T10:46:40.521104Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Successful notarization using tcp! session_id="eb313aa0-dcde-4f0d-851b-33b7859ebb3a" elapsed_time_millis=12856
2025-08-08T10:46:40.553698Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:46:40.553809Z  INFO tokio-runtime-worker ThreadId(02) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:46:40.553899Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:46:40.554261Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service: Received upgrade protocol request
2025-08-08T10:46:40.554321Z DEBUG tokio-runtime-worker ThreadId(31) notary_server::service::tcp: Upgraded to tcp connection session_id="ddbabb4c-e36e-44b3-b305-003b7d6c1b20"
2025-08-08T10:46:40.554335Z DEBUG tokio-runtime-worker ThreadId(31) notary_server::service: Starting notarization... session_id="ddbabb4c-e36e-44b3-b305-003b7d6c1b20"
2025-08-08T10:46:40.649645Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:46:46.406939Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:46:46.406969Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:46:47.115290Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:46:47.280088Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:47.706914Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:46:47.789385Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:47.953044Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:46:47.953081Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:46:48.035443Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544003Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544042Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544062Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544078Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544093Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544108Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544123Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544129Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544133Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544139Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544143Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544155Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544188Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544202Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544207Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544211Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544216Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544220Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.544223Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.544227Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.715997Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.716034Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.716201Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.716225Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.716435Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.716456Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.716573Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.716605Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.717001Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.717019Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.717275Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.717296Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.717429Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.717446Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.717611Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.717631Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.717852Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.717869Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.718280Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.718301Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.718307Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:46:48.718318Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:46:48.718503Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:46:49.009108Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:46:49.009161Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:46:49.036755Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:46:52.783336Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:46:52.890769Z ERROR tokio-runtime-worker ThreadId(31) verifier:verify: tlsn_verifier: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:46:52.890799Z ERROR tokio-runtime-worker ThreadId(31) verifier:notarize: tlsn_verifier: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:46:53.012669Z ERROR tokio-runtime-worker ThreadId(31) verifier:notarize: tlsn_verifier: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:46:53.012724Z ERROR tokio-runtime-worker ThreadId(31) notary_server::service::tcp: Failed notarization using tcp: Error occurred during notarization: verifier error: commit error caused by: I/O error: frame size too big session_id="ddbabb4c-e36e-44b3-b305-003b7d6c1b20" elapsed_time_millis=12458
```
