FROM adrianbrink/rust:nightly

RUN mkdir -p /source
ADD . /source
WORKDIR /source

CMD ["cargo", "run"]