FROM rust:slim-bullseye
RUN apt-get update
RUN apt-get install \
  tesseract-ocr \
  libtesseract-dev \
  libasound2 \
  libsdl2-dev \
  clang \
  build-essential \
  pkg-config \
  android-sdk \
  -y

# update crates.io index to speedup development compile time
RUN cargo search --limit=0

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# RUN . "$HOME/.cargo/env" && rustup install stable

WORKDIR /usr/app/

# initialize adb for keeping the key between runs
RUN adb devices

COPY ./assets /usr/app/assets
COPY ./Cargo.lock ./Cargo.toml /usr/app/

RUN mkdir src &&\
  echo "fn main(){}" > src/main.rs &&\
  cargo build --release &&\
  rm -r ./src/

COPY ./src/ /usr/app/src

# this is to force cargo to rebuild
RUN touch src/main.rs
RUN cargo build --release

CMD ["cargo", "run", "--release"]
