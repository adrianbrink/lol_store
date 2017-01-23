FROM adrianbrink/rust:nightly

RUN apt-get update && \
    apt-get install -y \
    libssl-dev libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /source
ADD . /source
WORKDIR /source