FROM ubuntu:latest

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /usr/app
ADD ./target/debug/sakura /usr/app

CMD [ "/usr/app/sakura" ]