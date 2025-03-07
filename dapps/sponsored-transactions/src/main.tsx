// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { MysClientProvider, WalletProvider } from '@mysten/dapp-kit';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import ReactDOM from 'react-dom/client';

import { App } from './App';

import '@mysten/dapp-kit/dist/index.css';
import './index.css';

import { getFullnodeUrl } from '@mysten/mys/client';

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<QueryClientProvider client={queryClient}>
			<MysClientProvider
				defaultNetwork="testnet"
				networks={{ testnet: { url: getFullnodeUrl('testnet') } }}
			>
				<WalletProvider enableUnsafeBurner>
					<App />
				</WalletProvider>
			</MysClientProvider>
		</QueryClientProvider>
	</React.StrictMode>,
);
