FROM gramineproject/gramine:v1.5

RUN apt-get update
RUN apt-get install -y make gcc

ENV SGX 1

RUN gramine-sgx-gen-private-key

WORKDIR /root/

ADD . ./

RUN mkdir -p untrustedhost

RUN SGX=1 make

ENTRYPOINT []
CMD [ "gramine-sgx-sigstruct-view", "synedrion-sgx-rpc-server.sig" ]
