# Version v0.1.0-alpha.8

## Execution logs

### Prover (test.sh)

```
URI: /16kb.json
Starting an MPC TLS connection with the server
Got a response from the server: 200 OK
Notarization complete!
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.

URI: /32kb.json
Starting an MPC TLS connection with the server
2025-08-08T10:26:49.501170Z ERROR mpc_tls::leader: error=internal error: "state error: must be in active or closed state to flush record layer"
Error: hyper::Error(IncompleteMessage)
```

### Notary (notary.sh)

```
2025-08-08T10:26:37.721187Z DEBUG main ThreadId(01) notary_server: Server config loaded settings.config=NotaryServerProperties { server: ServerProperties { name: "notary-server", host: "0.0.0.0", port: 7047, html_info: "<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"author\" content=\"tlsnotary\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n</head>\n<body>\n  <svg width=\"86\" height=\"88\" viewBox=\"0 0 86 88\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n    <path d=\"M25.5484 0.708986C25.5484 0.17436 26.1196 -0.167376 26.5923 0.0844205L33.6891 3.86446C33.9202 3.98756 34.0645 4.22766 34.0645 4.48902V9.44049H37.6129C38.0048 9.44049 38.3226 9.75747 38.3226 10.1485V21.4766L36.1936 20.0606V11.5645H34.0645V80.9919C34.0645 81.1134 34.0332 81.2328 33.9735 81.3388L30.4251 87.6388C30.1539 88.1204 29.459 88.1204 29.1878 87.6388L25.6394 81.3388C25.5797 81.2328 25.5484 81.1134 25.5484 80.9919V0.708986Z\" fill=\"#243F5F\"/>\n    <path d=\"M21.2903 25.7246V76.7012H12.7742V34.2207H0V25.7246H21.2903Z\" fill=\"#243F5F\"/>\n    <path d=\"M63.871 76.7012H72.3871V34.2207H76.6452V76.7012H85.1613V25.7246H63.871V76.7012Z\" fill=\"#243F5F\"/>\n    <path d=\"M38.3226 25.7246H59.6129V34.2207H46.8387V46.9649H59.6129V76.7012H38.3226V68.2051H51.0968V55.4609H38.3226V25.7246Z\" fill=\"#243F5F\"/>\n  </svg>\n  <h1>Notary Server {version}!</h1>\n  <ul>\n    <li>public key: <pre>{public_key}</pre></li>\n    <li>git commit hash: <a href=\"https://github.com/tlsnotary/tlsn/commit/{git_commit_hash}\">{git_commit_hash}</a></li>\n    <li><a href=\"healthcheck\">health check</a></li>\n    <li><a href=\"info\">info</a></li>\n  </ul>\n</body>\n" }, notarization: NotarizationProperties { max_sent_data: 1024, max_recv_data: 33792, timeout: 1800 }, tls: TLSProperties { enabled: false, private_key_pem_path: Some(""), certificate_pem_path: Some("") }, notary_key: NotarySigningKeyProperties { private_key_pem_path: "/notary/notary.key", public_key_pem_path: "/notary/notary.pub" }, logging: LoggingProperties { level: "DEBUG", filter: None }, authorization: AuthorizationProperties { enabled: false, whitelist_csv_path: Some("") } }
2025-08-08T10:26:37.721235Z DEBUG main ThreadId(01) run_server: notary_server::server: Loading notary server's signing key
2025-08-08T10:26:37.721652Z DEBUG main ThreadId(01) run_server: notary_server::server: Successfully loaded notary server's signing key!
2025-08-08T10:26:37.721762Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping TLS setup as it is turned off.
2025-08-08T10:26:37.721776Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping authorization as it is turned off.
2025-08-08T10:26:37.721797Z  INFO main ThreadId(01) run_server: notary_server::server: Listening for TCP traffic at 0.0.0.0:7047
2025-08-08T10:26:40.052325Z DEBUG main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:26:40.052418Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:26:40.052523Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(33792) }))
2025-08-08T10:26:40.052865Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:26:40.052964Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="8c316315-4541-4aa0-ae80-e0806a5147ce"
2025-08-08T10:26:40.052980Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="8c316315-4541-4aa0-ae80-e0806a5147ce"
2025-08-08T10:26:41.248415Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:26:43.134279Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:26:43.134316Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:26:44.060311Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:26:44.567037Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:26:44.691032Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:26:45.076754Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:26:45.076866Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:26:45.076917Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:26:45.323530Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:26:45.323569Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:26:45.325042Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:26:46.309917Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:26:46.376132Z  INFO tokio-runtime-worker ThreadId(02) verifier:finalize: tlsn_verifier::notarize: Sent attestation
2025-08-08T10:26:46.429472Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Successful notarization using tcp! session_id="8c316315-4541-4aa0-ae80-e0806a5147ce"
2025-08-08T10:26:46.467345Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:26:46.467402Z  INFO tokio-runtime-worker ThreadId(02) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:26:46.467492Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(33792) }))
2025-08-08T10:26:46.467875Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service: Received upgrade protocol request
2025-08-08T10:26:46.467932Z DEBUG tokio-runtime-worker ThreadId(31) notary_server::service::tcp: Upgraded to tcp connection session_id="961de51e-c667-464f-ac6d-cdbb518c534c"
2025-08-08T10:26:46.467947Z DEBUG tokio-runtime-worker ThreadId(31) notary_server::service: Starting notarization... session_id="961de51e-c667-464f-ac6d-cdbb518c534c"
2025-08-08T10:26:47.064848Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:26:49.033698Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:26:49.033724Z  INFO tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:26:49.501627Z ERROR tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: error=I/O error: unexpected end of file
2025-08-08T10:26:49.560861Z ERROR tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: error=verifier error: mpc error caused by: I/O error: unexpected end of file
2025-08-08T10:26:49.560897Z ERROR tokio-runtime-worker ThreadId(02) verifier:notarize: tlsn_verifier: error=verifier error: mpc error caused by: I/O error: unexpected end of file
2025-08-08T10:26:49.560908Z ERROR tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Failed notarization using tcp: Error occurred during notarization: verifier error: mpc error caused by: I/O error: unexpected end of file session_id="961de51e-c667-464f-ac6d-cdbb518c534c"
```
