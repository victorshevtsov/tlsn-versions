# Version v0.1.0-alpha.6

## Execution logs

### Prover (test.sh)

```
URI: /512kb.json
Starting an MPC TLS connection with the server
Got a response from the server
Notarization completed successfully!
The proof has been written to `simple_proof.json`

URI: /1024kb.json
Starting an MPC TLS connection with the server
Got a response from the server
Notarization completed successfully!
The proof has been written to `simple_proof.json`
```

### Notary (notary.sh)

```
2025-08-08T09:59:44.394347Z DEBUG main ThreadId(01) notary_server: Server config loaded config=NotaryServerProperties { server: ServerProperties { name: "notary-server", host: "0.0.0.0", port: 7047, html_info: "<h1>Notary Server {version}!</h1>\n<ul>\n<li>git commit hash: <a href=\"https://github.com/tlsnotary/tlsn/commit/{git_commit_hash}\">{git_commit_hash}</a></li>\n<li>git commit timestamp: {git_commit_timestamp}</li>\n<li>public key: <pre>{public_key}</pre></li>\n</ul>\n<a href=\"/healthcheck\">health check</a> - <a href=\"/info\">info</a><br/>\n" }, notarization: NotarizationProperties { max_transcript_size: 1050624 }, tls: TLSProperties { enabled: false, private_key_pem_path: "", certificate_pem_path: "" }, notary_key: NotarySigningKeyProperties { private_key_pem_path: "/notary/notary.key", public_key_pem_path: "/notary/notary.pub" }, logging: LoggingProperties { level: "DEBUG", filter: None }, authorization: AuthorizationProperties { enabled: false, whitelist_csv_path: "" } }
2025-08-08T09:59:44.394395Z DEBUG main ThreadId(01) run_server: notary_server::server: Loading notary server's signing key
2025-08-08T09:59:44.394725Z DEBUG main ThreadId(01) run_server: notary_server::server: Successfully loaded notary server's signing key!
2025-08-08T09:59:44.394918Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping TLS setup as it is turned off.
2025-08-08T09:59:44.394935Z DEBUG main ThreadId(01) run_server: notary_server::server: Skipping authorization as it is turned off.
2025-08-08T09:59:44.394980Z  INFO main ThreadId(01) run_server: notary_server::server: Listening for TCP traffic at 0.0.0.0:7047
2025-08-08T10:00:57.184652Z DEBUG main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:00:57.184821Z  INFO tokio-runtime-worker ThreadId(33) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:00:57.185174Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(1049600) }))
2025-08-08T10:00:57.185805Z  INFO tokio-runtime-worker ThreadId(02) notary_server::service: Received upgrade protocol request
2025-08-08T10:00:57.185958Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service::tcp: Upgraded to tcp connection session_id="3bb3d6fb-691d-4755-b03c-eafb40316625"
2025-08-08T10:00:57.185991Z DEBUG tokio-runtime-worker ThreadId(02) notary_server::service: Starting notarization... session_id="3bb3d6fb-691d-4755-b03c-eafb40316625"
2025-08-08T10:01:03.779757Z DEBUG tokio-runtime-worker ThreadId(02) setup_mpc_backend: tlsn_verifier::tls: MPC backend setup complete
2025-08-08T10:01:06.999511Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:01:10.249451Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:01:13.606467Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:01:17.046384Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:01:20.305783Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:01:23.829210Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:01:27.302538Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:01:30.621386Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:01:33.959554Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:01:37.237426Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:01:41.032414Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:01:44.561509Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:01:48.097674Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:01:51.381517Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:01:54.735428Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:01:58.086471Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:02:01.349297Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:02:04.578546Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:02:07.847405Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:02:11.134298Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:02:15.353404Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:02:19.070468Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:02:22.339481Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:02:25.726812Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:02:29.614834Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:02:32.945582Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:02:36.320281Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:02:39.589444Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:02:43.172341Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:02:46.506434Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:02:50.186578Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:02:53.421477Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:02:56.966340Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:02:57.991953Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: leader committed transcript
2025-08-08T10:02:58.542477Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: follower actor stopped
2025-08-08T10:02:58.542822Z  INFO tokio-runtime-worker ThreadId(31) tlsn_verifier::tls: Finished TLS session
2025-08-08T10:02:58.579060Z DEBUG tokio-runtime-worker ThreadId(31) finalize: tlsn_verifier::tls::notarize: revealed OT secret
2025-08-08T10:03:30.099046Z  INFO tokio-runtime-worker ThreadId(32) finalize: tlsn_verifier::tls::notarize: Finalized all MPC
2025-08-08T10:03:30.099228Z  INFO tokio-runtime-worker ThreadId(32) finalize: tlsn_verifier::tls::notarize: Signed session header
2025-08-08T10:03:30.099247Z  INFO tokio-runtime-worker ThreadId(32) finalize: tlsn_verifier::tls::notarize: Sent session header
2025-08-08T10:03:30.247246Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service::tcp: Successful notarization using tcp! session_id="3bb3d6fb-691d-4755-b03c-eafb40316625"
2025-08-08T10:03:30.793097Z DEBUG                 main ThreadId(01) run_server: notary_server::server: Received a prover's TCP connection
2025-08-08T10:03:30.793164Z  INFO tokio-runtime-worker ThreadId(32) notary_server::server: Accepted prover's TCP connection
2025-08-08T10:03:30.917056Z  INFO tokio-runtime-worker ThreadId(32) notary_server::service: Received request for initializing a notarization session payload=Ok(Json(NotarizationSessionRequest { client_type: Tcp, max_sent_data: Some(1024), max_recv_data: Some(1049600) }))
2025-08-08T10:03:30.917581Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service: Received upgrade protocol request
2025-08-08T10:03:30.917647Z DEBUG tokio-runtime-worker ThreadId(31) notary_server::service::tcp: Upgraded to tcp connection session_id="eb5a285d-aef1-4989-b53b-2ed46aceb7ba"
2025-08-08T10:03:30.917663Z DEBUG tokio-runtime-worker ThreadId(31) notary_server::service: Starting notarization... session_id="eb5a285d-aef1-4989-b53b-2ed46aceb7ba"
2025-08-08T10:03:37.046557Z DEBUG tokio-runtime-worker ThreadId(32) setup_mpc_backend: tlsn_verifier::tls: MPC backend setup complete
2025-08-08T10:03:40.265466Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:03:43.438652Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:03:46.949475Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:03:50.455432Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:03:53.724430Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:03:57.132325Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:04:00.556280Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:04:03.899406Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:04:07.269440Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:04:10.467407Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:04:14.369358Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:04:17.821448Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:04:21.268427Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:04:24.463400Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:04:27.813495Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:04:31.188527Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:04:34.564451Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:04:37.782408Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:04:40.982499Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:04:44.133830Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:04:48.169291Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:04:51.766397Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:04:55.004453Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:04:58.249530Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:05:02.102431Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:05:05.296424Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:05:08.598390Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:05:11.812434Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:05:15.306497Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:05:18.661283Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:05:22.263438Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:05:25.546443Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:05:29.012404Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:05:32.252545Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:05:35.472476Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:05:38.733403Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:05:41.944868Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:05:45.159433Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:05:48.344457Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:05:53.957019Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:05:57.465417Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:06:01.953397Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:06:05.229405Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:06:08.666323Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:06:12.052372Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:06:15.401369Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:06:18.719379Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:06:22.021434Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:06:26.612387Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:06:30.048378Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:06:33.432365Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:06:36.795399Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:06:40.132416Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:06:43.552815Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:06:46.862420Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:06:50.206376Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:06:54.141415Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:06:57.465493Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:07:00.793459Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:07:04.081442Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:07:08.034433Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:07:11.344406Z DEBUG tokio-runtime-worker ThreadId(02) tls_mpc::follower: decrypting message
2025-08-08T10:07:14.577337Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:07:17.875445Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: decrypting message
2025-08-08T10:07:21.558338Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: decrypting message
2025-08-08T10:07:22.384145Z DEBUG tokio-runtime-worker ThreadId(31) tls_mpc::follower: leader committed transcript
2025-08-08T10:07:23.663616Z DEBUG tokio-runtime-worker ThreadId(32) tls_mpc::follower: follower actor stopped
2025-08-08T10:07:23.665075Z  INFO tokio-runtime-worker ThreadId(32) tlsn_verifier::tls: Finished TLS session
2025-08-08T10:07:23.706839Z DEBUG tokio-runtime-worker ThreadId(32) finalize: tlsn_verifier::tls::notarize: revealed OT secret
2025-08-08T10:08:27.455169Z  INFO tokio-runtime-worker ThreadId(31) finalize: tlsn_verifier::tls::notarize: Finalized all MPC
2025-08-08T10:08:27.456004Z  INFO tokio-runtime-worker ThreadId(31) finalize: tlsn_verifier::tls::notarize: Signed session header
2025-08-08T10:08:27.456031Z  INFO tokio-runtime-worker ThreadId(31) finalize: tlsn_verifier::tls::notarize: Sent session header
2025-08-08T10:08:27.466470Z  INFO tokio-runtime-worker ThreadId(31) notary_server::service::tcp: Successful notarization using tcp! session_id="eb5a285d-aef1-4989-b53b-2ed46aceb7ba"
```
