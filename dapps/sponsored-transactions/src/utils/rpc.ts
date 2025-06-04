// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import { getFullnodeUrl, MysClient } from '@socialproof/mys/client';

export const client = new MysClient({ url: getFullnodeUrl('testnet') });
