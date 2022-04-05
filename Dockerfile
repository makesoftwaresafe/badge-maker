FROM fuzzers/cargo-fuzz:0.10.0

WORKDIR  /work
ADD . .

RUN cargo fuzz build

# Set to fuzz!
ENTRYPOINT []
CMD ["/work/fuzz/target/x86_64-unknown-linux-gnu/release/svg"]