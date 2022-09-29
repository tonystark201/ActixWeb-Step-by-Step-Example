FROM rust:latest

RUN apt-get update -yqq && apt-get install -yqq cmake g++ iputils-ping lsof nginx supervisor 
RUN cargo install diesel_cli --no-default-features --features postgres
RUN mkdir -p /etc/supervisor /home/service/logs

WORKDIR /home/service 
COPY . /home/service
COPY supervisor.conf /etc/supervisor/conf.d/supervisord.conf

RUN echo REDIS=redis://172.21.0.7:6379/1 > /home/service/.env
RUN echo DATABASE_URL=postgres://postgres:postgres@172.21.0.8:5432/postgres >> /home/service/.env

RUN cargo build --release 
EXPOSE 22 80 8081 8082 8083
CMD ["/usr/bin/supervisord"]
