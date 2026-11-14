#!/bin/sh
# Chromium's sandbox needs user namespaces, which Docker's default seccomp profile blocks. The
# container is the isolation boundary here, and the browser only ever opens Wealthsimple's login.
exec /usr/bin/chromium --no-sandbox --disable-dev-shm-usage "$@"
