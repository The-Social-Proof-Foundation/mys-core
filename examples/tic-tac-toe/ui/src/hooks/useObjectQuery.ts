// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useMysClientContext, useMysClientQuery, UseMysClientQueryOptions } from '@mysten/dapp-kit';
import { GetObjectParams, MysObjectResponse } from '@mysten/mys/client';
import { useQueryClient, UseQueryResult } from '@tanstack/react-query';

export type UseObjectQueryOptions = UseMysClientQueryOptions<'getObject', MysObjectResponse>;
export type UseObjectQueryResponse = UseQueryResult<MysObjectResponse, Error>;
export type InvalidateUseObjectQuery = () => void;

/**
 * Fetches an object, returning the response from RPC and a callback
 * to invalidate it.
 */
export function useObjectQuery(
	params: GetObjectParams,
	options?: UseObjectQueryOptions,
): [UseObjectQueryResponse, InvalidateUseObjectQuery] {
	const ctx = useMysClientContext();
	const client = useQueryClient();
	const response = useMysClientQuery('getObject', params, options);

	const invalidate = async () => {
		await client.invalidateQueries({
			queryKey: [ctx.network, 'getObject', params],
		});
	};

	return [response, invalidate];
}
