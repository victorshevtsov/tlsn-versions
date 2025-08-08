// This example demonstrates how to use the Prover to acquire an attestation for
// an HTTP request sent to example.com. The attestation and secrets are saved to
// disk.

use http_body_util::Empty;
use hyper::{body::Bytes, Request, StatusCode};
use hyper_util::rt::TokioIo;
use notary_client::{Accepted, NotarizationRequest, NotaryClient};
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};

use tlsn_common::config::ProtocolConfig;
use tlsn_core::{request::RequestConfig, transcript::TranscriptCommitConfig};
use tlsn_formats::http::{DefaultHttpCommitter, HttpCommit, HttpTranscript};
use tlsn_prover::{Prover, ProverConfig};

// Setting of the application server
const SERVER_DOMAIN: &str = "mock.verity.usher.so";
// const SERVER_DOMAIN: &str = "example.com";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

const MAX_SENT_DATA: usize = 1024;
const MAX_RECV_DATA: usize = 1024 * 1024 + 1024; // payload + headers

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let uri: String = std::env::var("URI").unwrap_or("/1kb.json".into());
    println!("URI: {}", uri);

    let notary_host: String = std::env::var("NOTARY_HOST").unwrap_or("127.0.0.1".into());
    let notary_port: u16 = std::env::var("NOTARY_PORT")
        .map(|port| port.parse().expect("port should be valid integer"))
        .unwrap_or(7047);

    // Build a client to connect to the notary server.
    let notary_client = NotaryClient::builder()
        .host(notary_host)
        .port(notary_port)
        // WARNING: Always use TLS to connect to notary server, except if notary is running locally
        // e.g. this example, hence `enable_tls` is set to False (else it always defaults to True).
        .enable_tls(false)
        .build()
        .unwrap();

    // Send requests for configuration and notarization to the notary server.
    let notarization_request = NotarizationRequest::builder()
        // We must configure the amount of data we expect to exchange beforehand, which will
        // be preprocessed prior to the connection. Reducing these limits will improve
        // performance.
        .max_sent_data(MAX_SENT_DATA)
        .max_recv_data(MAX_RECV_DATA)
        .build()?;

    let Accepted {
        io: notary_connection,
        id: _session_id,
        ..
    } = notary_client
        .request_notarization(notarization_request)
        .await
        .expect("Could not connect to notary. Make sure it is running.");

    // let (prover_socket, notary_socket) = tokio::io::duplex(1 << 16);

    // Start a local simple notary service
    // tokio::spawn(run_notary(notary_connection.compat()));

    // Prover configuration.
    let config = ProverConfig::builder()
        .server_name(SERVER_DOMAIN)
        .protocol_config(
            ProtocolConfig::builder()
                // We must configure the amount of data we expect to exchange beforehand, which will
                // be preprocessed prior to the connection. Reducing these limits will improve
                // performance.
                .max_sent_data(MAX_SENT_DATA)
                .max_recv_data(MAX_RECV_DATA)
                .build()?,
        )
        .build()?;

    // Create a new prover and perform necessary setup.
    let prover = Prover::new(config)
        .setup(notary_connection.compat())
        .await?;

    // Open a TCP connection to the server.
    let client_socket = tokio::net::TcpStream::connect((SERVER_DOMAIN, 443)).await?;

    // Bind the prover to the server connection.
    // The returned `mpc_tls_connection` is an MPC TLS connection to the server: all
    // data written to/read from it will be encrypted/decrypted using MPC with
    // the notary.
    let (mpc_tls_connection, prover_fut) = prover.connect(client_socket.compat()).await?;
    let mpc_tls_connection = TokioIo::new(mpc_tls_connection.compat());

    // Spawn the prover task to be run concurrently in the background.
    let prover_task = tokio::spawn(prover_fut);

    // Attach the hyper HTTP client to the connection.
    let (mut request_sender, connection) =
        hyper::client::conn::http1::handshake(mpc_tls_connection).await?;

    // Spawn the HTTP task to be run concurrently in the background.
    tokio::spawn(connection);

    // Build a simple HTTP request with common headers
    let request = Request::builder()
        .uri(uri)
        .header("Host", SERVER_DOMAIN)
        .header("Accept", "*/*")
        // Using "identity" instructs the Server not to use compression for its HTTP response.
        // TLSNotary tooling does not support compression.
        .header("Accept-Encoding", "identity")
        .header("Connection", "close")
        .header("User-Agent", USER_AGENT)
        .body(Empty::<Bytes>::new())?;

    println!("Starting an MPC TLS connection with the server");

    // Send the request to the server and wait for the response.
    let response = request_sender.send_request(request).await?;

    println!("Got a response from the server");

    assert!(response.status() == StatusCode::OK);

    // The prover task should be done now, so we can await it.
    let prover = prover_task.await??;

    // Prepare for notarization.
    let mut prover = prover.start_notarize();

    // Parse the HTTP transcript.
    let transcript = HttpTranscript::parse(prover.transcript())?;

    // Commit to the transcript.
    let mut builder = TranscriptCommitConfig::builder(prover.transcript());

    DefaultHttpCommitter::default().commit_transcript(&mut builder, &transcript)?;

    prover.transcript_commit(builder.build()?);

    // Request an attestation.
    let config = RequestConfig::default();

    let (attestation, secrets) = prover.finalize(&config).await?;

    // Write the attestation to disk.
    tokio::fs::write(
        "example.attestation.tlsn",
        bincode::serialize(&attestation)?,
    )
    .await?;

    // Write the secrets to disk.
    tokio::fs::write("example.secrets.tlsn", bincode::serialize(&secrets)?).await?;

    println!("Notarization completed successfully!");
    println!(
        "The attestation has been written to `example.attestation.tlsn` and the \
        corresponding secrets to `example.secrets.tlsn`."
    );
    println!("");

    Ok(())
}
