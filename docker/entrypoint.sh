#!/bin/sh
# Container entrypoint for ws-mcp.
#   serve ...          the MCP server (stdio by default; pass --transport=http for HTTP)
#   login --viewer     sign in through Chromium on a virtual display, viewable at
#                      http://127.0.0.1:6080/vnc_lite.html when the container's port 6080 is published
# Anything else is passed to ws-mcp unchanged.
set -eu

if [ "${1:-}" = "login" ] && [ "${2:-}" = "--viewer" ]; then
  shift 2
  export DISPLAY=:99
  Xvfb :99 -screen 0 1280x1024x24 -nolisten tcp >/dev/null 2>&1 &
  for _ in $(seq 50); do [ -e /tmp/.X11-unix/X99 ] && break; sleep 0.1; done
  fluxbox >/dev/null 2>&1 &
  x11vnc -display :99 -localhost -nopw -forever -shared -quiet >/dev/null 2>&1 &
  websockify --web /usr/share/novnc 6080 localhost:5900 >/dev/null 2>&1 &
  sleep 1
  echo "Open http://127.0.0.1:6080/vnc_lite.html?autoconnect=true to sign in to Wealthsimple."
  exec ws-mcp login "$@"
fi

exec ws-mcp "$@"
