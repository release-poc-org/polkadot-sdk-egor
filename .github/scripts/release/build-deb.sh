#!/usr/bin/env bash

PRODUCT=$1
PROFILE=${PROFILE:-production}

cargo install cargo-deb
echo "Using cargo-deb v$(cargo-deb --version)"
echo "Building a Debian package for '$PRODUCT' in '$PROFILE' profile"

cargo deb --profile $PROFILE --no-strip --no-build -p $PRODUCT

deb=target/debian/$PRODUCT_*_amd64.deb

cp $deb /artifacts/$PRODUCT/
