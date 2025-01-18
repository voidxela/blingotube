# https://dioxuslabs.com/learn/0.6/guide/deploy/#building-a-dockerfile
FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS prep
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=prep /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli -y --force
RUN dx bundle --platform web

FROM chef AS app
COPY --from=build /app/target/dx/blingotube/release/web/ /usr/local/app
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080
WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
