// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import '@socialproof/dapp-kit/dist/index.css';
import './index.css';
import '@fontsource-variable/inter';
import '@fontsource-variable/red-hat-mono';

import { MysClientProvider, WalletProvider } from '@socialproof/dapp-kit';
import { getFullnodeUrl } from '@socialproof/mys/client';
import { QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { queryClient } from './lib/queryClient';
import { router } from './routes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<QueryClientProvider client={queryClient}>
			<MysClientProvider
				defaultNetwork="mys:mainnet"
				networks={{
					'mys:testnet': { url: getFullnodeUrl('testnet') },
					'mys:mainnet': { url: getFullnodeUrl('mainnet') },
					'mys:devnet': { url: getFullnodeUrl('devnet') },
				}}
			>
				<WalletProvider>
					<RouterProvider router={router} />
				</WalletProvider>
			</MysClientProvider>
		</QueryClientProvider>
	</React.StrictMode>,
);
