// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import { Toaster } from 'react-hot-toast';
import { Outlet } from 'react-router-dom';

import { Header } from '@/components/header';
import { Warning } from '@/components/warning';

export function Root() {
	return (
		<div>
			<Toaster position="bottom-center" />
			<Header />
			<div className="container py-8">
				<Outlet />
			</div>
			<Warning />
		</div>
	);
}
