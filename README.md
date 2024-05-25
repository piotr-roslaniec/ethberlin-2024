# synedrion-tee-rpc-server

# Installation

Follow the official [Gramine instructions](https://gramine.readthedocs.io/en/stable/installation.html)

Also, run:

```sh
gramine-sgx-gen-private-key
```

# Running

```sh
# Build the project and generate manifest
make SGX=1

# Start the server
make SGX=1 start-gramine-server
```

To test with non-SGX Gramine instead, omit `SGX=1` in both commands.

# To Do

- [x] Setting up Gramine
- [x] Running Synedrion end-to-end in a single-participant setup
- [ ] Producing a quote and a report for specific user data
- [ ] Configure remote attestation using DCAP
- [ ] Using the quote and report to prove origin of a secret share for Synedrion participant
- [ ] Running Synedrion end-to-end in a multi-participant setup
- [ ] (Optional) Run & terminate TLS communication in the enclave
