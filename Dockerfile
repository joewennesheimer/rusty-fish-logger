#Build Stage

FROM rust:latest as builder

WORKDIR /app

# Accept thhe build arguments
ARG DB_USER
ARG DB_PASSWORD

ENV DB_USER $DB_USER
ENV DB_PASSWORD $DB_PASSWORD

ARG DB_URL
ENV DB_URL $DB_URL

COPY . . 

RUN cargo install --path .

RUN cargo build --release

# Production Stage
FROM debian

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/fish-logger .

CMD ["./fish-logger"]