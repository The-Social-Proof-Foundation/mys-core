// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/**
 * Takes an object of { key: value } and builds a URL param string.
 * e.g. { page: 1, limit: 10 } -> ?page=1&limit=10
 */
export const constructUrlSearchParams = (
  object: Record<string, string>,
): string => {
  const searchParams = new URLSearchParams();

  for (const key in object) {
    searchParams.set(key, object[key]);
  }

  return `?${searchParams.toString()}`;
};

/** Checks whether we have a next page */
export const getNextPageParam = (lastPage: any) => {
  if ("api" in lastPage) {
    return lastPage.api.cursor;
  }
  return lastPage.cursor;
};

/**
 * Format an address for display in social proof tokens configuration tables.
 * Shows 8 characters on each side instead of the default 12.
 */
export function formatSptConfigAddress(address: string): string {
  if (!address || address.length < 18) {
    return address;
  }

  // Remove 0x prefix if present
  const withoutPrefix = address.startsWith("0x") ? address.slice(2) : address;
  
  if (withoutPrefix.length <= 16) {
    return address;
  }

  const start = withoutPrefix.slice(0, 8);
  const end = withoutPrefix.slice(-8);
  return `0x${start}...${end}`;
}

/**
 * Abbreviated header labels for social proof tokens configuration table
 */
export const SPT_CONFIG_HEADERS: Record<string, string> = {
  "Transaction ID": "Txn ID",
  "Ecosystem Treasury": "Ecosystem Treasury",
  "Updated By": "Updated By",
  "Last Updated": "Last Updated",
};

/**
 * Get abbreviated header label, or return original if not found
 */
export function getSptConfigHeaderLabel(label: string): string {
  return SPT_CONFIG_HEADERS[label] || label;
}
