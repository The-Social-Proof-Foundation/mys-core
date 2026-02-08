// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --simulator

// Testing various input errors for the MysNS name resolution:
// 1. Not enough labels (need at least two)
// 2. Too long
// 3. Bad (inconsistent) use of separators
// 4. Indvidual label too short
// 5. Individual label too long
// 6, 7, 8, 9. Bad characters (non-alphanumeric, or leading/trailing hyphen)

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["foo"]
}

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.myso"]
}

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["foo*bar.myso"]
}

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["foo..myso"]
}

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["toolongtoolongtoolongtoolongtoolongtoolongtoolongtoolongtoolongtoolong.myso"]
}

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["-foo.myso"]
}

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["foo-.myso"]
}

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["foo_bar.myso"]
}

//# run-jsonrpc
{
  "method": "mysx_resolveNameServiceAddress",
  "params": ["🫠.myso"]
}
