FROM rust:latest

RUN apt-get update -yqq && apt-get install -yqq cmake g++ iputils-ping lsof
RUN cargo install diesel_cli --no-default-features --features postgres

RUN mkdir -p /home/service/logs

WORKDIR /home/service 
COPY . /home/service

RUN echo REDIS=redis://172.20.0.7:6379/1 > /home/service/.env
RUN echo DATABASE_URL=postgres://postgres:postgres@172.20.0.8:5432/postgres >> /home/service/.env

# add --release later on
RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]
