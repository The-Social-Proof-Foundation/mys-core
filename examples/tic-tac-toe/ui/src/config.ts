// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import { createNetworkConfig } from '@socialproof/dapp-kit';
import { getFullnodeUrl } from '@socialproof/mys/client';

import DevnetPackage from './env.devnet.ts';
import LocalnetPackage from './env.localnet.ts';
import MainnetPackage from './env.mainnet.ts';
import TestnetPackage from './env.testnet.ts';

const { networkConfig, useNetworkVariable } = createNetworkConfig({
	localnet: {
		url: getFullnodeUrl('localnet'),
		variables: {
			explorer: (id: string) => `https://explorer.polymedia.app/object/${id}/?network=local`,
			...LocalnetPackage,
		},
	},
	devnet: {
		url: getFullnodeUrl('devnet'),
		variables: {
			explorer: (id: string) => `https://mysscan.xyz/devnet/object/${id}/`,
			...DevnetPackage,
		},
	},
	testnet: {
		url: getFullnodeUrl('testnet'),
		variables: {
			explorer: (id: string) => `https://mysscan.xyz/testnet/object/${id}/`,
			...TestnetPackage,
		},
	},
	mainnet: {
		url: getFullnodeUrl('mainnet'),
		variables: {
			explorer: (id: string) => `https://mysscan.xyz/mainnet/object/${id}/`,
			...MainnetPackage,
		},
	},
});

export { networkConfig, useNetworkVariable };
