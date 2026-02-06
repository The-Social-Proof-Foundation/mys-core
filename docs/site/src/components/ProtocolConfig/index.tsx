// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import React, { useState, useEffect } from "react";
import Tabs from "@theme/Tabs";
import TabItem from "@theme/TabItem";
import axios from "axios";

export default function ProtocolConfig(props) {
  const data = {
    jsonrpc: "2.0",
    id: 1,
    method: "mys_getProtocolConfig",
    params: [],
  };
  const urls = [
    "https://fullnode.mainnet.mysocial.network:443",
    "https://fullnode.testnet.mysocial.network:443",
    "https://fullnode.devnet.mysocial.network:443",
  ];
  const [results, setResults] = useState({
    mainnet: null,
    testnet: null,
    devnet: null,
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const { fields } = props;

  const parseResult = (data) => {
    let items = Object.entries(data);

    return items.map((item) => {
      if (item[1] === null) {
        return item;
      }
      if (typeof item[1] === "object") {
        const [k, v] = Object.entries(item[1])[0];
        return [item[0], k, v];
      }
      return item;
    });
  };

  const DisplayResults = (props) => {
    const { results } = props;
    return (
      <>
        <style>{`
          .config-table {
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
            width: 100%;
          }
          .config-row {
            display: grid;
            grid-template-columns: minmax(min-content, max-content) 1fr;
            gap: 1rem;
            align-items: start;
            padding: 0.75rem 0;
            border-bottom: 1px solid var(--ifm-color-emphasis-200);
          }
          .config-row:last-child {
            border-bottom: none;
          }
          .config-label {
            font-weight: 600;
            color: var(--ifm-color-content);
            white-space: nowrap;
            min-width: fit-content;
          }
          .config-value {
            color: var(--ifm-color-content-secondary);
            word-break: break-word;
            overflow-wrap: anywhere;
            min-width: 0;
          }
          .config-value code {
            word-break: break-all;
            white-space: pre-wrap;
          }
        `}</style>
        <div className="config-table">
          {results.map((item, index) => (
            <>
              {(!fields || fields.includes(item[0])) && (
                <div key={index} className="config-row">
                  <div className="config-label">{item[0]}</div>
                  <div className="config-value">
                    {item[2] ? item[2] : "null"}
                  </div>
                </div>
              )}
            </>
          ))}
        </div>
      </>
    );
  };

  useEffect(() => {
    const fetchData = async () => {
      try {
        const responses = await Promise.all(
          urls.map((url) =>
            axios.post(url, data, {
              headers: {
                "Content-Type": "application/json",
              },
            }),
          ),
        );

        setResults({
          mainnet: parseResult(responses[0].data.result.attributes),
          testnet: parseResult(responses[1].data.result.attributes),
          devnet: parseResult(responses[2].data.result.attributes),
        });
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>Error: {error}</div>;
  }

  return (
    <Tabs groupId="mys-network">
      <TabItem value="mainnet" label="Mainnet">
        <DisplayResults results={results.mainnet} />
      </TabItem>
      <TabItem value="testnet" label="Testnet">
        <DisplayResults results={results.testnet} />
      </TabItem>
      <TabItem value="devnet" label="Devnet">
        <DisplayResults results={results.devnet} />
      </TabItem>
    </Tabs>
  );
}
