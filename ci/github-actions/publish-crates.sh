#! /bin/bash
set -o errexit -o nounset

if [ -z "$RELEASE_TAG" ]; then
	echo '::error::Environment variable RELEASE_TAG is required but missing'
	exit 1
fi

echo "Publishing rounded-div@$RELEASE_TAG..."
exec cargo publish
