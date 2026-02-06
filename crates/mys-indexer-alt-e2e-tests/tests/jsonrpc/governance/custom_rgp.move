// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 70 --simulator --objects-snapshot-min-checkpoint-lag 2 --reference-gas-price 1337

//# run-jsonrpc
{
  "method": "mysx_getReferenceGasPrice",
  "params": []
}
