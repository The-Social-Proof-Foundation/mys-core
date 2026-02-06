// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

const standards = [
	'standards',
	'standards/coin',
	{
		type: 'category',
		label: 'Closed-Loop Token',
		link: {
			type: 'doc',
			id: 'standards/closed-loop-token',
		},
		items: [
			'standards/closed-loop-token/action-request',
			'standards/closed-loop-token/token-policy',
			'standards/closed-loop-token/spending',
			'standards/closed-loop-token/rules',
			'standards/closed-loop-token/coin-token-comparison',
		],
	},
	{
		type: 'category',
		label: 'OrderBook',
		link: {
			type: 'doc',
			id: 'standards/orderbook',
		},
		items: [
			{
				type: 'category',
				label: 'OrderBookV3',
				link: {
					type: 'doc',
					id: 'standards/orderbookv3',
				},
				items: [
					'standards/orderbookv3/design',
					'standards/orderbookv3/balance-manager',
					'standards/orderbookv3/query-the-pool',
					'standards/orderbookv3/orders',
					'standards/orderbookv3/swaps',
					'standards/orderbookv3/flash-loans',
					'standards/orderbookv3/staking-governance',
				],
			},
			'standards/orderbookv3-indexer',
			{
				type: 'category',
				label: 'OrderBookV3 SDK',
				link: {
					type: 'doc',
					id: 'standards/orderbookv3-sdk',
				},
				items: [
					'standards/orderbookv3-sdk/flash-loans',
					'standards/orderbookv3-sdk/orders',
					'standards/orderbookv3-sdk/pools',
					'standards/orderbookv3-sdk/staking-governance',
					'standards/orderbookv3-sdk/swaps',
				],
			},
			{
				type: 'category',
				label: 'OrderBookV2',
				link: {
					type: 'doc',
					id: 'standards/orderbookv2',
				},
				items: [
					'standards/orderbookv2/design',
					'standards/orderbookv2/orders',
					'standards/orderbookv2/pools',
					'standards/orderbookv2/query-the-pool',
					'standards/orderbookv2/routing-a-swap',
					'standards/orderbookv2/trade-and-swap',
				],
			},
		],
	},
	'standards/display',
	'standards/wallet-standard',
];
module.exports = standards;
