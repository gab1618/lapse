FROM rust:1.97 AS builder

WORKDIR /repo

COPY . .

RUN cargo build --release

FROM archlinux

COPY --from=builder /repo/target/release/lapse /bin
