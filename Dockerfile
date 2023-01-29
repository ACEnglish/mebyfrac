FROM rust:latest

RUN apt-get -qq update \
  && DEBIAN_FRONTEND=noninteractive apt-get install -yq \
  vim

WORKDIR /data
ENTRYPOINT ["/bin/bash"]
