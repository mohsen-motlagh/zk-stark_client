#!/bin/sh
# wait-for-server.sh

# The first argument should be the URL to check, including the path to a health check endpoint if available
URL="$1"
shift
CMD="$@"

echo "Waiting for server at $URL to be available..."

# Try until the curl command succeeds
until curl -sf $URL > /dev/null; do
    >&2 echo "Server is unavailable - sleeping"
    sleep 1
done

>&2 echo "Server is up - executing command"
exec $CMD
