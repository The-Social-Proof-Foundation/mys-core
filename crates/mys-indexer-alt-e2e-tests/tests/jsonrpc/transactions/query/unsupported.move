// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 70 --addresses test=0x0 --simulator

//# create-checkpoint

//# run-jsonrpc
{
  "method": "mysx_queryTransactionBlocks",
  "params": [
    {
      "filter": {
        "TransactionKind": "NotSupported"
      }
    }
  ]
}
