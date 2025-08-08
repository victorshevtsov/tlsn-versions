# Version v0.1.0-alpha.7

## Execution logs

### Prover (test.sh)

```
URI: /512kb.json
Starting an MPC TLS connection with the server
Got a response from the server
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.

URI: /1024kb.json
Starting an MPC TLS connection with the server
Got a response from the server
Notarization completed successfully!
The attestation has been written to `example.attestation.tlsn` and the corresponding secrets to `example.secrets.tlsn`.
```

### Notary (notary.sh)

```
2025-08-08T10:12:44.064028Z DEBUG main ThreadId(01) notary_server: Server config loaded config=NotaryServerProperties { server: ServerProperties { name: "notary-server", host: "0.0.0.0", port: 7047, html_info: "<h1>Notary Server {version}!</h1>\n<ul>\n<li>git commit hash: <a href=\"https://github.com/tlsnotary/tlsn/commit/{git_commit_hash}\">{git_commit_hash}</a></li>\n<li>git commit timestamp: {git_commit_timestamp}</li>\n<li>public key: <pre>{public_key}</pre></li>\n</ul>\n<a href=\"/healthcheck\">health check</a> - <a href=\"/info\">info</a><br/>\n" }, notarization: NotarizationProperties { max_sent_data: 1024, max_recv_data: 1049600 }, tls: TLSProperties { enabled: false, private_key_pem_path: "", certificate_pem_path: "" }, notary_key: NotarySigningKeyProperties { private_key_pem_path: "/notary/notary.key", public_key_pem_path: "/notary/notary.pub" }, logging: LoggingProperties { level: "DEBUG", filter: None }, authorization: AuthorizationProperties { enabled: false, whitelist_csv_path: "" } }
2025-08-08T10:12:44.064081Z DEBUG main ThreadId(01) run_server: notary_server::server: Loading notary server's signing key
2025-08-08T10:12:44.064555Z DEBUG main ThreadId(01) run_server: notary_server::server: Successfully loaded notary server's signing key!
2025-08-08T10:12:44.064653Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping TLS setup as it is turned off.
2025-08-08T10:12:44.064667Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping authorization as it is turned off.
2025-08-08T10:12:44.064688Z  INFO main ThreadId(01) run_server: notary_server::server: Listening for TCP traffic at 0.0.0.0:7047
2025-08-08T10:13:27.770783Z DEBUG main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:13:27.770871Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:13:27.770992Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(1049600) }))
2025-08-08T10:13:27.771340Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:13:27.771461Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="63282337-1059-4646-a604-dd54ac1515e4"
2025-08-08T10:13:27.771477Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="63282337-1059-4646-a604-dd54ac1515e4"
2025-08-08T10:13:27.774693Z DEBUG tokio-runtime-worker ThreadId(32) verifier:setup:setup_mpc_backend: tlsn_verifier: starting MPC backend setup
2025-08-08T10:13:31.414377Z DEBUG tokio-runtime-worker ThreadId(02) verifier:setup:setup_mpc_backend: tlsn_verifier: MPC backend setup complete
2025-08-08T10:13:36.308011Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: leader committed transcript
2025-08-08T10:13:36.405915Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:38.705378Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:40.993341Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:43.305336Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:45.739287Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:47.977404Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:50.329593Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:52.587406Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:55.264518Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:57.462403Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:13:59.665362Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:01.912293Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:04.370481Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:06.721390Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:09.058443Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:11.687359Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:14.208436Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:16.450576Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:18.689439Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:20.953464Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:23.127363Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:25.302378Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:27.546360Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:29.807451Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:32.492353Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:34.688522Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:36.849327Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:39.092390Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:41.440403Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:43.686380Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:46.962396Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:49.123374Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:51.948603Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:14:53.929961Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: follower actor stopped
2025-08-08T10:14:53.930282Z  INFO tokio-runtime-worker ThreadId(32) verifier:run: tlsn_verifier: Finished TLS session
2025-08-08T10:14:53.948814Z DEBUG tokio-runtime-worker ThreadId(32) verifier:finalize: tlsn_verifier::notarize: revealed OT secret
2025-08-08T10:15:21.948799Z  INFO tokio-runtime-worker ThreadId(02) verifier:finalize: tlsn_verifier::notarize: Finalized all MPC
2025-08-08T10:15:21.949076Z  INFO tokio-runtime-worker ThreadId(02) verifier:finalize: tlsn_verifier::notarize: Sent session header
2025-08-08T10:15:21.949342Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Successful notarization using tcp! session_id="63282337-1059-4646-a604-dd54ac1515e4"
2025-08-08T10:15:22.443615Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:15:22.443674Z  INFO tokio-runtime-worker ThreadId(02) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:15:22.444721Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(1049600) }))
2025-08-08T10:15:22.445063Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received upgrade protocol request
2025-08-08T10:15:22.445121Z DEBUG tokio-runtime-worker ThreadId(32) notary_server::service::tcp: Upgraded to tcp connection session_id="6e668766-b371-4166-8f91-4bc6512b0e22"
2025-08-08T10:15:22.445135Z DEBUG tokio-runtime-worker ThreadId(32) notary_server::service: Starting notarization... session_id="6e668766-b371-4166-8f91-4bc6512b0e22"
2025-08-08T10:15:22.445658Z DEBUG tokio-runtime-worker ThreadId(32) verifier:setup:setup_mpc_backend: tlsn_verifier: starting MPC backend setup
2025-08-08T10:15:26.079337Z DEBUG tokio-runtime-worker ThreadId(31) verifier:setup:setup_mpc_backend: tlsn_verifier: MPC backend setup complete
2025-08-08T10:15:31.984904Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: leader committed transcript
2025-08-08T10:15:32.080882Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:34.244383Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:36.515442Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:38.825445Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:41.311566Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:43.527440Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:45.844521Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:48.056614Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:50.741370Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:53.022327Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:55.339454Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:15:57.682445Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:00.192475Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:02.414366Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:04.751487Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:07.487640Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:10.004467Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:12.228350Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:14.339243Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:16.456510Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:18.631390Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:20.829356Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:22.969424Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:25.092362Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:27.579371Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:29.752362Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:31.920485Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:34.101458Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:36.216348Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:38.321349Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:41.453323Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:43.629464Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:46.325353Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:48.439556Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:50.541326Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:52.713910Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:54.943360Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:57.123376Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:16:59.296487Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:01.404498Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:03.544423Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:05.729362Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:07.941359Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:10.128365Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:12.253354Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:14.404543Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:16.617533Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:18.781451Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:21.896407Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:24.001565Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:26.154499Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:28.322374Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:30.539424Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:32.697364Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:34.902391Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:37.006368Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:39.273405Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:41.449355Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:43.656412Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:45.830363Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:50.524354Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:52.744924Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:54.957282Z DEBUG tokio-runtime-worker ThreadId(32) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:17:57.110397Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:18:00.472248Z DEBUG tokio-runtime-worker ThreadId(02) verifier:run: tls_mpc::follower: decrypting message
2025-08-08T10:18:03.263843Z DEBUG tokio-runtime-worker ThreadId(31) verifier:run: tls_mpc::follower: follower actor stopped
2025-08-08T10:18:03.263958Z  INFO tokio-runtime-worker ThreadId(31) verifier:run: tlsn_verifier: Finished TLS session
2025-08-08T10:18:03.282835Z DEBUG tokio-runtime-worker ThreadId(31) verifier:finalize: tlsn_verifier::notarize: revealed OT secret
2025-08-08T10:18:58.460696Z  INFO tokio-runtime-worker ThreadId(02) verifier:finalize: tlsn_verifier::notarize: Finalized all MPC
2025-08-08T10:18:58.463622Z  INFO tokio-runtime-worker ThreadId(02) verifier:finalize: tlsn_verifier::notarize: Sent session header
2025-08-08T10:18:58.465022Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Successful notarization using tcp! session_id="6e668766-b371-4166-8f91-4bc6512b0e22"
```
