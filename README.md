# TLSN Versions

This repository contains a collection of tests for evaluating different versions of TLSNotary. Each subdirectory corresponds to a specific version and includes everything needed for evaluation. All directories share the same structure:

- `notary.sh` – Script that builds a Docker image for the notary server of the corresponding version and starts the server
- `test.sh` – Script that builds a sample prover and runs a series of tests requesting resources of expected payload sizes
- `README.md` – Contains execution logs for the Notary and Prover

To perform a test, use two terminals:

- In `terminal 1`, navigate to the directory for the desired version and run `notary.sh`
- In `terminal 2`, navigate to the same directory and run `test.sh`
