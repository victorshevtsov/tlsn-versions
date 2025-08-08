# Version v0.1.0-alpha.9

## Execution logs

### Prover (test.sh)

```
URI: /32kb.json
Starting an MPC TLS connection with the server
Got a response from the server: 200 OK
Notarization complete!
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.

URI: /63kb.json
Starting an MPC TLS connection with the server
Got a response from the server: 200 OK
Notarization complete!
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.

URI: /64kb.json
Starting an MPC TLS connection with the server
Got a response from the server: 200 OK
Error: ProverError { kind: Mpc, source: Some(VmError(Execute(VmError(Execute(Kind(UnexpectedEof)))))) }
```

### Notary (notary.sh)

```
2025-08-08T10:29:04.362582Z DEBUG main ThreadId(01) notary_server: Server config loaded settings.config=NotaryServerProperties { server: ServerProperties { name: "notary-server", host: "0.0.0.0", port: 7047, html_info: "<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"author\" content=\"tlsnotary\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n</head>\n<body>\n  <svg width=\"86\" height=\"88\" viewBox=\"0 0 86 88\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n    <path d=\"M25.5484 0.708986C25.5484 0.17436 26.1196 -0.167376 26.5923 0.0844205L33.6891 3.86446C33.9202 3.98756 34.0645 4.22766 34.0645 4.48902V9.44049H37.6129C38.0048 9.44049 38.3226 9.75747 38.3226 10.1485V21.4766L36.1936 20.0606V11.5645H34.0645V80.9919C34.0645 81.1134 34.0332 81.2328 33.9735 81.3388L30.4251 87.6388C30.1539 88.1204 29.459 88.1204 29.1878 87.6388L25.6394 81.3388C25.5797 81.2328 25.5484 81.1134 25.5484 80.9919V0.708986Z\" fill=\"#243F5F\"/>\n    <path d=\"M21.2903 25.7246V76.7012H12.7742V34.2207H0V25.7246H21.2903Z\" fill=\"#243F5F\"/>\n    <path d=\"M63.871 76.7012H72.3871V34.2207H76.6452V76.7012H85.1613V25.7246H63.871V76.7012Z\" fill=\"#243F5F\"/>\n    <path d=\"M38.3226 25.7246H59.6129V34.2207H46.8387V46.9649H59.6129V76.7012H38.3226V68.2051H51.0968V55.4609H38.3226V25.7246Z\" fill=\"#243F5F\"/>\n  </svg>\n  <h1>Notary Server {version}!</h1>\n  <ul>\n    <li>public key: <pre>{public_key}</pre></li>\n    <li>git commit hash: <a href=\"https://github.com/tlsnotary/tlsn/commit/{git_commit_hash}\">{git_commit_hash}</a></li>\n    <li><a href=\"healthcheck\">health check</a></li>\n    <li><a href=\"info\">info</a></li>\n  </ul>\n</body>\n" }, notarization: NotarizationProperties { max_sent_data: 1024, max_recv_data: 132096, timeout: 1800 }, tls: TLSProperties { enabled: false, private_key_pem_path: Some(""), certificate_pem_path: Some("") }, notary_key: NotarySigningKeyProperties { private_key_pem_path: "/notary/notary.key", public_key_pem_path: "/notary/notary.pub" }, logging: LoggingProperties { level: "DEBUG", filter: None, format: Compact }, authorization: AuthorizationProperties { enabled: false, whitelist_csv_path: Some("") } }
2025-08-08T10:29:04.362645Z DEBUG main ThreadId(01) run_server: notary_server::server: Loading notary server's signing key
2025-08-08T10:29:04.363118Z DEBUG main ThreadId(01) run_server: notary_server::server: Successfully loaded notary server's signing key!
2025-08-08T10:29:04.363214Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping TLS setup as it is turned off.
2025-08-08T10:29:04.363228Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping authorization as it is turned off.
2025-08-08T10:29:04.363248Z  INFO main ThreadId(01) run_server: notary_server::server: Listening for TCP traffic at 0.0.0.0:7047
2025-08-08T10:29:13.539201Z DEBUG main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:29:13.539292Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:29:13.539394Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:29:13.539678Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:29:13.539791Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="fe1a0fba-75f8-4b65-a717-1fb2f9ed0a38"
2025-08-08T10:29:13.539807Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="fe1a0fba-75f8-4b65-a717-1fb2f9ed0a38"
2025-08-08T10:29:14.780974Z DEBUG tokio-runtime-worker ThreadId(28) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:29:20.560145Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:29:20.560183Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:29:21.490048Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:29:21.572071Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:21.998841Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:29:22.081067Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.123017Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:29:22.123046Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:29:22.204959Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.590325Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.590360Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.590369Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.590386Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.590701Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.590722Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.590952Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.590985Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.591115Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.591144Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.591272Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.591297Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.591682Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.591709Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.591925Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.591949Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.591957Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:22.591976Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:22.592074Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:29:22.838614Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:29:22.838663Z  INFO tokio-runtime-worker ThreadId(28) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:29:22.841878Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:29:24.643531Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:29:24.778253Z  INFO tokio-runtime-worker ThreadId(31) verifier:finalize: tlsn_verifier::notarize: Sent attestation
2025-08-08T10:29:24.928748Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service::tcp: Successful notarization using tcp! session_id="fe1a0fba-75f8-4b65-a717-1fb2f9ed0a38"
2025-08-08T10:29:24.981595Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:29:24.981661Z  INFO tokio-runtime-worker ThreadId(31) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:29:24.981760Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:29:24.982127Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:29:24.982181Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="62e85644-2a83-4a45-afc4-ea0151e350e9"
2025-08-08T10:29:24.982195Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="62e85644-2a83-4a45-afc4-ea0151e350e9"
2025-08-08T10:29:25.660740Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:29:31.420987Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:29:31.421017Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:29:32.266241Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:29:32.348045Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:32.773594Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:29:32.856147Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:32.897841Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:29:32.897871Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:29:32.980132Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.365050Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.365085Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.365275Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.365291Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.365529Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.365546Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.365856Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.365873Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.366008Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.366031Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.366339Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.366363Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.366495Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.366513Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.366724Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.366770Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.366913Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.366930Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.366936Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.366940Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.367105Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.367123Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.367213Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.367229Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.367642Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.367665Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.576941Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.576985Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.577144Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.577167Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.577369Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.577393Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.578121Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.578141Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.579344Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.579363Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.579536Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.579556Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.579711Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.579719Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.580013Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.580045Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.580303Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.580319Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.580602Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.580619Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.580624Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:33.580638Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:33.580681Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:29:33.828578Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:29:33.828624Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:29:33.835015Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:29:36.851346Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:29:37.018017Z  INFO tokio-runtime-worker ThreadId(31) verifier:finalize: tlsn_verifier::notarize: Sent attestation
2025-08-08T10:29:37.222583Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service::tcp: Successful notarization using tcp! session_id="62e85644-2a83-4a45-afc4-ea0151e350e9"
2025-08-08T10:29:37.275931Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:29:37.276001Z  INFO tokio-runtime-worker ThreadId(31) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:29:37.276096Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:29:37.276370Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:29:37.276425Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="a2de792c-6526-4a51-b281-14187e3d1dc3"
2025-08-08T10:29:37.276442Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="a2de792c-6526-4a51-b281-14187e3d1dc3"
2025-08-08T10:29:37.948874Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:29:43.675780Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:29:43.675809Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:29:44.542982Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:29:44.625066Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.058666Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:29:45.141150Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.189225Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:29:45.189255Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:29:45.271045Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.660054Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.660085Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.660104Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.660120Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.660161Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.660178Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.660184Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.660188Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.660543Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.660567Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.660585Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.660601Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.660843Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.660865Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.661033Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.661053Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.872444Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.872481Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.872656Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.872679Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.872759Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.872775Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.873090Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.873118Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.873126Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.873143Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.873345Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.873355Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.873464Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.873492Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.873780Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.873842Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.873917Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.873937Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.873943Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.873947Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.874130Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.874140Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.874379Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.874401Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.874407Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:29:45.874420Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:29:45.874512Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:29:46.121592Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:29:46.121640Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:29:46.128398Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:29:49.141934Z DEBUG tokio-runtime-worker ThreadId(28) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:29:49.326910Z ERROR tokio-runtime-worker ThreadId(28) verifier:finalize: tlsn_verifier::notarize: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:29:49.326946Z ERROR tokio-runtime-worker ThreadId(28) verifier:notarize: tlsn_verifier: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:29:49.326978Z ERROR tokio-runtime-worker ThreadId(28) notary_server::service::tcp: Failed notarization using tcp: Error occurred during notarization: verifier error: commit error caused by: I/O error: frame size too big session_id="a2de792c-6526-4a51-b281-14187e3d1dc3"
```
