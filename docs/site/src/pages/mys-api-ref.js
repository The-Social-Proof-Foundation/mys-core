// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import React from "react";
import Layout from "@theme/Layout";
import API from "../components/API";

import useDocusaurusContext from "@docusaurus/useDocusaurusContext";

export default function JsonRpc() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout title={`MySocial API Reference | ${siteConfig.title}`}>
      <API />
    </Layout>
  );
}
