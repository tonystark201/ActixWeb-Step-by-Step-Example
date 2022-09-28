FROM rust:latest

RUN apt-get update -yqq && apt-get install -yqq cmake g++ iputils-ping lsof
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./css ./css
COPY ./javascript ./javascript
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./templates ./templates
COPY ./.env ./.env
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml

WORKDIR .

RUN echo REDIS=redis://172.10.0.5:6379/1 > .env
RUN echo DATABASE_URL=postgres://postgres:postgres@172.10.0.6:5432/postgres >> .env

# add --release later on
RUN cargo build --release

EXPOSE 8080

# "RUST_LOG='info,parser::expression=info,actix_web=info'"

#curl --header "Content-Type: application/json" --request GET  http://localhost/

# add --release later on
CMD ["cargo", "run", "--release"]
