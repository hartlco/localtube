FROM rust:1.52

WORKDIR /usr/src/localtube

COPY . .

RUN wget https://yt-dl.org/downloads/latest/youtube-dl -O /usr/local/bin/youtube-dl
RUN chmod a+rx /usr/local/bin/youtube-dl

RUN cargo install --path .

CMD ["localtube" , "/config/config.toml", "/config/downloaded.toml"]