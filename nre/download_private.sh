#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

if ! cosign version &> /dev/null
then
    echo "cosign in not installed, Please install cosign for binary verification."
    echo "https://docs.sigstore.dev/cosign/installation"
    exit
fi

commit_sha=$1
pub_key=https://mys-private.s3.us-west-2.amazonaws.com/mys_security_release.pem
url=https://mys-releases.s3-accelerate.amazonaws.com/$commit_sha

echo "[+] Downloading mys binaries for $commit_sha ..."
curl $url/mys -o mys
curl $url/mys-indexer -o mys-indexer
curl $url/mys-node -o mys-node
curl $url/mys-tool -o mys-tool

echo "[+] Verifying mys binaries for $commit_sha ..."
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/mys.sig mys
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/mys-indexer.sig mys-indexer
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/mys-node.sig mys-node
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/mys-tool.sig mys-tool
