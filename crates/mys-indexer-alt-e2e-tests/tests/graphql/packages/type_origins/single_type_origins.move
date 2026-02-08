// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --accounts A --addresses P1=0x0 --simulator

//# publish --upgradeable --sender A
module P1::M {
  public struct S1 { }
}

//# create-checkpoint

//# run-graphql
{
  package(address: "@{obj_1_0}") {
    typeOrigins {
        module
        struct
        definingId
    }
  }
}