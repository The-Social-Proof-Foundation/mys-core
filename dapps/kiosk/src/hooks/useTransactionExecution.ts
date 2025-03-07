// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useSignTransaction, useMysClient } from '@mysten/dapp-kit';
import { MysTransactionBlockResponseOptions } from '@mysten/mys/client';
import { Transaction } from '@mysten/mys/transactions';

// A helper to execute transactions by:
// 1. Signing them using the wallet
// 2. Executing them using the rpc provider
export function useTransactionExecution() {
	const provider = useMysClient();

	// sign transaction from the wallet
	const { mutateAsync: signTransaction } = useSignTransaction();

	// tx: Transaction
	const signAndExecute = async ({
		tx,
		options = { showEffects: true },
	}: {
		tx: Transaction;
		options?: MysTransactionBlockResponseOptions | undefined;
	}) => {
		const signedTx = await signTransaction({ transaction: tx });

		const res = await provider.executeTransactionBlock({
			transactionBlock: signedTx.bytes,
			signature: signedTx.signature,
			options,
		});

		const status = res.effects?.status?.status === 'success';

		if (status) return true;
		else throw new Error('Transaction execution failed.');
	};

	return { signAndExecute };
}
