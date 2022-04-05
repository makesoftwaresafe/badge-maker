FROM rustlang/rust:nightly-slim
RUN cargo install cargo-fuzz

WORKDIR  /work
ADD . .

RUN cargo fuzz build

# Set to fuzz!
ENTRYPOINT []
CMD ["/workdir/fuzz/target/x86_64-unknown-linux-gnu/release/svg"]