FROM liuchong/rustup:stable

RUN apt-get -y update
RUN apt-get -y install libsqlite3-dev

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV DATABASE_URL=./db/myblog.db

ADD . /app
VOLUME ['/db']

WORKDIR /app

RUN rustup default nightly

#RUN cargo build --release
RUN cargo build

EXPOSE 8080

#CMD ["cargo", "run", "--release"]
CMD ["cargo", "run"]