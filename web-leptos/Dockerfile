FROM rust:1.67

RUN cargo install --locked cargo-leptos

WORKDIR /usr/src/red-siren
COPY ../ .

RUN cd web-leptos/
RUN cargo leptos build
