FROM adrianbrink/rust:nightly

RUN mkdir -p /source
COPY . /source
WORKDIR /source

CMD ["/bin/bash"]