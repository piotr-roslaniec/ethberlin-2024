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

To test with non-SGX Gramine instead, omit `SGX=1` in both commands. You may want to use `docker compose` setup for running on SGX.

# Attestation

Recover `MRENCLAVE` by running:

```sh
docker build -t gramine .
docker run --rm gramine
```

Use `docker compose` to run on platforms with SGX support:

```sh
docker compose run --rm gramine gramine-sgx synedrion-sgx-rpc-server
```

# To Do

- [x] Setting up Gramine
- [x] Running Synedrion end-to-end in a single-participant setup
- [x] Benchmark the overhead of running Synedrion in an enclave
- [ ] Producing a quote and a report for specific user data
- [ ] Configure remote attestation using DCAP
- [ ] Using the quote and report to prove the origin of a secret share for Synedrion participant
- [ ] Running Synedrion end-to-end in a multi-participant setup
- [ ] (Optional) Run & terminate TLS communication in the enclave

# "Benchmarks"

- In enclave (SGX=1):   2.486434s
- In simulated enclave: 2.033924s

TODO: Benchmark Synedrion steps separately

# References

Docker setup adapted from [amiller/gramine-rsademo](https://github.com/amiller/gramine-rsademo)
