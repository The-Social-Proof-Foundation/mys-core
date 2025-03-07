// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getFullnodeUrl, MysClient } from '@mysten/mys/client';

export const client = new MysClient({ url: getFullnodeUrl('testnet') });
