// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --accounts A --simulator

//# run-graphql
{
  checkpoint { sequenceNumber }
}

//# create-checkpoint

//# run-graphql
{
  checkpoint { sequenceNumber }
}

//# create-checkpoint

//# run-graphql
{
  checkpoint { sequenceNumber }
}

//# run-graphql
{
  # Test that explicit null defaults to latest checkpoint
  checkpoint(sequenceNumber: null) { sequenceNumber }
}
