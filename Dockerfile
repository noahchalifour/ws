# ws-mcp in a container: the MCP server over streamable HTTP, plus Chromium on a virtual display for
# `login --viewer`. The session is a file on a volume ($WS_SESSION_FILE), since containers have no keychain.
#
#   docker run -v ws-mcp:/home/ws-mcp -p 127.0.0.1:6080:6080 -it IMAGE login --viewer
#   docker run -v ws-mcp:/home/ws-mcp -p 127.0.0.1:3000:3000 IMAGE \
#     serve --transport=http --host=0.0.0.0 --port=3000 --allowed-host=localhost
FROM rust:1.99.0-slim-trixie AS build
RUN apt-get update \
 && apt-get install -y --no-install-recommends cmake clang libclang-dev perl pkg-config make g++ git \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p ws-mcp \
 && cp target/release/ws-mcp /usr/local/bin/ws-mcp

FROM debian:trixie-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      ca-certificates chromium fonts-liberation xvfb x11vnc fluxbox novnc python3-websockify tini \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --uid 1000 --create-home --home-dir /home/ws-mcp ws-mcp
COPY --from=build /usr/local/bin/ws-mcp /usr/local/bin/ws-mcp
COPY docker/entrypoint.sh /usr/local/bin/docker-entrypoint
COPY docker/chromium.sh /usr/local/bin/chromium-container
ENV HOME=/home/ws-mcp \
    WS_SESSION_FILE=/home/ws-mcp/session.json \
    WS_MODE=read \
    CHROME=/usr/local/bin/chromium-container
USER 1000:1000
WORKDIR /home/ws-mcp
VOLUME /home/ws-mcp
EXPOSE 3000 6080
ENTRYPOINT ["tini", "--", "docker-entrypoint"]
CMD ["serve", "--transport=http", "--host=0.0.0.0", "--port=3000"]
