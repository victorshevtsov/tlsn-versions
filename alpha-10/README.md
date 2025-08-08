# Version v0.1.0-alpha.10

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
2025-08-08T10:38:30.690686Z DEBUG main ThreadId(01) notary_server: Server config loaded settings.config=NotaryServerProperties { server: ServerProperties { name: "notary-server", host: "0.0.0.0", port: 7047, html_info: "<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"author\" content=\"tlsnotary\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n</head>\n<body>\n  <svg width=\"86\" height=\"88\" viewBox=\"0 0 86 88\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n    <path d=\"M25.5484 0.708986C25.5484 0.17436 26.1196 -0.167376 26.5923 0.0844205L33.6891 3.86446C33.9202 3.98756 34.0645 4.22766 34.0645 4.48902V9.44049H37.6129C38.0048 9.44049 38.3226 9.75747 38.3226 10.1485V21.4766L36.1936 20.0606V11.5645H34.0645V80.9919C34.0645 81.1134 34.0332 81.2328 33.9735 81.3388L30.4251 87.6388C30.1539 88.1204 29.459 88.1204 29.1878 87.6388L25.6394 81.3388C25.5797 81.2328 25.5484 81.1134 25.5484 80.9919V0.708986Z\" fill=\"#243F5F\"/>\n    <path d=\"M21.2903 25.7246V76.7012H12.7742V34.2207H0V25.7246H21.2903Z\" fill=\"#243F5F\"/>\n    <path d=\"M63.871 76.7012H72.3871V34.2207H76.6452V76.7012H85.1613V25.7246H63.871V76.7012Z\" fill=\"#243F5F\"/>\n    <path d=\"M38.3226 25.7246H59.6129V34.2207H46.8387V46.9649H59.6129V76.7012H38.3226V68.2051H51.0968V55.4609H38.3226V25.7246Z\" fill=\"#243F5F\"/>\n  </svg>\n  <h1>Notary Server {version}!</h1>\n  <ul>\n    <li>public key: <pre>{public_key}</pre></li>\n    <li>git commit hash: <a href=\"https://github.com/tlsnotary/tlsn/commit/{git_commit_hash}\">{git_commit_hash}</a></li>\n    <li><a href=\"healthcheck\">health check</a></li>\n    <li><a href=\"info\">info</a></li>\n  </ul>\n</body>\n" }, notarization: NotarizationProperties { max_sent_data: 1024, max_recv_data: 132096, timeout: 1800 }, tls: TLSProperties { enabled: false, private_key_pem_path: Some(""), certificate_pem_path: Some("") }, notary_key: NotarySigningKeyProperties { private_key_pem_path: "/notary/notary.key", public_key_pem_path: "/notary/notary.pub" }, logging: LoggingProperties { level: "DEBUG", filter: None, format: Compact }, authorization: AuthorizationProperties { enabled: false, whitelist_csv_path: Some("") }, concurrency: 32 }
2025-08-08T10:38:30.690750Z DEBUG main ThreadId(01) run_server: notary_server::server: Loading notary server's signing key
2025-08-08T10:38:30.691222Z DEBUG main ThreadId(01) run_server: notary_server::server: Successfully loaded notary server's signing key!
2025-08-08T10:38:30.691322Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping TLS setup as it is turned off.
2025-08-08T10:38:30.691337Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping authorization as it is turned off.
2025-08-08T10:38:30.691357Z  INFO main ThreadId(01) run_server: notary_server::server: Listening for TCP traffic at 0.0.0.0:7047
2025-08-08T10:38:34.163249Z DEBUG main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:38:34.163337Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:38:34.163441Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:38:34.163762Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:38:34.163865Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="b0e3ba9f-0d75-4f06-9c77-9553cd0f2430"
2025-08-08T10:38:34.163882Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="b0e3ba9f-0d75-4f06-9c77-9553cd0f2430"
2025-08-08T10:38:35.457193Z DEBUG tokio-runtime-worker ThreadId(32) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:38:41.368163Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:38:41.368205Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:38:42.246053Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:38:42.328127Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:42.753090Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:38:42.835064Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:42.886133Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:38:42.886163Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:38:42.968166Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.354689Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:43.354718Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.354784Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:43.354804Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.355010Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:43.355019Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.355342Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:43.355363Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.355557Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:43.355587Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.355816Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:43.355826Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.356105Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:43.356131Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.356154Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:43.356172Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:43.356210Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:38:43.603853Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:38:43.603907Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:38:43.608233Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:38:45.411874Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:38:45.563167Z  INFO tokio-runtime-worker ThreadId(32) verifier:finalize: tlsn_verifier::notarize: Sent attestation
2025-08-08T10:38:45.711001Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service::tcp: Successful notarization using tcp! session_id="b0e3ba9f-0d75-4f06-9c77-9553cd0f2430" elapsed_time_millis=11547
2025-08-08T10:38:45.753822Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:38:45.753886Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:38:45.753962Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:38:45.754271Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:38:45.754325Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="5afc0dbc-15e0-426c-b6c7-726521eadb86"
2025-08-08T10:38:45.754340Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="5afc0dbc-15e0-426c-b6c7-726521eadb86"
2025-08-08T10:38:46.439917Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:38:52.143427Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:38:52.143456Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:38:53.011313Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:38:53.093102Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:53.524338Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:38:53.606237Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:53.648139Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:38:53.648177Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:38:53.730118Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.116721Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.116790Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.116811Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.116827Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.117165Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.117187Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.117422Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.117434Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.117625Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.117643Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.117825Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.117845Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.118010Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.118022Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.118027Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.118030Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.118222Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.118245Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.118383Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.118406Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.118647Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.118666Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.328333Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.328370Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.328496Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.328518Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.328767Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.328792Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.329094Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.329114Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.329119Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.329132Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.329144Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.329150Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.329191Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.329207Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.329409Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.329433Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.329641Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.329661Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.329798Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.329825Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.330133Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.330155Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.330160Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:38:54.330173Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:38:54.330276Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:38:54.577792Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:38:54.577838Z  INFO tokio-runtime-worker ThreadId(32) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:38:54.586591Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:38:57.572851Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:38:57.779131Z  INFO tokio-runtime-worker ThreadId(32) verifier:finalize: tlsn_verifier::notarize: Sent attestation
2025-08-08T10:38:57.935785Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service::tcp: Successful notarization using tcp! session_id="5afc0dbc-15e0-426c-b6c7-726521eadb86" elapsed_time_millis=12181
2025-08-08T10:38:57.993185Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:38:57.993260Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:38:57.993390Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(132096) }))
2025-08-08T10:38:57.993808Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:38:57.993872Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="7294f185-aaf5-4e8d-b1ff-8db633195cea"
2025-08-08T10:38:57.993887Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="7294f185-aaf5-4e8d-b1ff-8db633195cea"
2025-08-08T10:38:58.705155Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup: tlsn_verifier: setting up mpc-tls
2025-08-08T10:39:04.582317Z DEBUG tokio-runtime-worker ThreadId(32) verifier:setup: tlsn_verifier: mpc-tls setup complete
2025-08-08T10:39:04.582346Z  INFO tokio-runtime-worker ThreadId(32) verifier:run: tlsn_verifier: starting MPC-TLS
2025-08-08T10:39:05.443145Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:39:05.525298Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:05.938089Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: processing 0 encrypt ops and 1 decrypt ops is_decrypting=false
2025-08-08T10:39:06.020192Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.061844Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::record_layer: started processing application data
2025-08-08T10:39:06.061878Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: processing 1 encrypt ops and 0 decrypt ops is_decrypting=false
2025-08-08T10:39:06.144085Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.520442Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.520471Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.520490Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.520506Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.520666Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.520718Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.520895Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.520932Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.521298Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.521322Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.521467Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.521487Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.521674Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.521683Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.521894Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.521912Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.522179Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.522199Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.726925Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.726955Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.726963Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.726979Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.727364Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.727392Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.727646Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.727668Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.727831Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.727851Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.728044Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.728064Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.728368Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.728404Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.728481Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.728502Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.728757Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.728775Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.729040Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.729058Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.729072Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run:flush: mpc_tls::record_layer: no operations to process, skipping is_decrypting=false
2025-08-08T10:39:06.729078Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run:run: mpc_tls::follower: flushed record layer
2025-08-08T10:39:06.729152Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committing
2025-08-08T10:39:06.976701Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run:run: mpc_tls::follower: committed
2025-08-08T10:39:06.976764Z  INFO tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finished MPC-TLS
2025-08-08T10:39:06.985287Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: finalizing mpc
2025-08-08T10:39:09.984334Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tlsn_verifier: mpc finalized
2025-08-08T10:39:10.174670Z ERROR tokio-runtime-worker ThreadId(02) verifier:finalize: tlsn_verifier::notarize: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:39:10.174715Z ERROR tokio-runtime-worker ThreadId(02) verifier:notarize: tlsn_verifier: error=verifier error: commit error caused by: I/O error: frame size too big
2025-08-08T10:39:10.174756Z ERROR tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Failed notarization using tcp: Error occurred during notarization: verifier error: commit error caused by: I/O error: frame size too big session_id="7294f185-aaf5-4e8d-b1ff-8db633195cea" elapsed_time_millis=12180
```
