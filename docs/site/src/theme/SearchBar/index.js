// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import React from "react";
import SearchBar from "@theme-original/SearchBar";

export default class SearchBarWrapper extends React.Component {
  componentDidMount() {
    try {
      window.initCookbook();
    } catch (e) {
      // Gracefully ignore errors if something goes wrong
      console.error("Erorr initializing Ask Cookbook", e);
    }
  }

  render() {
    return (
      <>
        <SearchBar {...this.props} />
      </>
    );
  }
}
