#!/bin/sh

FILE_PATH=$(dirname $(realpath $0))

# Export node options
export NODE_OPTIONS="--openssl-legacy-provider"

# Run server
cd $FILE_PATH/www
npm run start