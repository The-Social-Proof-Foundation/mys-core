// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  corePlugins: {
    preflight: false, // disable Tailwind's reset
  },
  content: ["./src/**/*.{js,jsx,ts,tsx}", "./docs/**/*.mdx"], // my markdown stuff is in ../docs, not /src
  darkMode: ["class", '[data-theme="dark"]'], // hooks into docusaurus' dark mode settings
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", ...defaultTheme.fontFamily.sans],
        twkeverett: ["Twkeverett"],
      },
      colors: {
        "mys-black": "var(--mys-black)",
        "mys-blue-primary": "rgb(var(--mys-blue-primary)/<alpha-value>)",
        "mys-blue": "var(--mys-blue)",
        "mys-blue-bright": "var(--mys-blue-bright)",
        "mys-blue-light": "var(--mys-blue-light)",
        "mys-blue-lighter": "var(--mys-blue-lighter)",
        "mys-blue-dark": "rgb(var(--mys-blue-dark)/<alpha-value>)",
        "mys-blue-darker": "var(--mys-blue-darker)",
        "mys-hero": "var(--mys-hero)",
        "mys-hero-dark": "var(--mys-hero-dark)",
        "mys-steel": "var(--mys-steel)",
        "mys-steel-dark": "var(--mys-steel-dark)",
        "mys-steel-darker": "var(--mys-steel-darker)",
        "mys-header-nav": "var(--mys-header-nav)",
        "mys-success": "var(--mys-success)",
        "mys-success-dark": "var(--mys-success-dark)",
        "mys-success-light": "var(--mys-success-light)",
        "mys-issue": "var(--mys-issue)",
        "mys-issue-dark": "var(--mys-issue-dark)",
        "mys-issue-light": "var(--mys-issue-light)",
        "mys-warning": "var(--mys-warning)",
        "mys-warning-dark": "var(--mys-warning-dark)",
        "mys-warning-light": "var(--mys-warning-light)",
        "mys-code": "var(--mys-code)",
        "mys-gray": {
          35: "var(--mys-gray-35)",
          40: "var(--mys-gray-40)",
          45: "var(--mys-gray-45)",
          50: "var(--mys-gray-50)",
          55: "var(--mys-gray-55)",
          60: "var(--mys-gray-60)",
          65: "var(--mys-gray-65)",
          70: "var(--mys-gray-70)",
          75: "var(--mys-gray-75)",
          80: "var(--mys-gray-80)",
          85: "var(--mys-gray-85)",
          90: "var(--mys-gray-90)",
          95: "var(--mys-gray-95)",
          100: "var(--mys-gray-100)",
        },
        "mys-grey": {
          35: "var(--mys-gray-35)",
          40: "var(--mys-gray-40)",
          45: "var(--mys-gray-45)",
          50: "var(--mys-gray-50)",
          55: "var(--mys-gray-55)",
          60: "var(--mys-gray-60)",
          65: "var(--mys-gray-65)",
          70: "var(--mys-gray-70)",
          75: "var(--mys-gray-75)",
          80: "var(--mys-gray-80)",
          85: "var(--mys-gray-85)",
          90: "var(--mys-gray-90)",
          95: "var(--mys-gray-95)",
          100: "var(--mys-gray-100)",
        },
        "mys-link-color-dark": "var(--mys-link-color-dark)",
        "mys-link-color-light": "var(--mys-link-color-light)",
        "mys-ghost-white": "var(--mys-ghost-white)",
        "mys-ghost-dark": "var(--mys-ghost-dark)",
        "ifm-background-color-dark": "var(--ifm-background-color-dark)",
        "mys-white": "rgb(var(--mys-white)/<alpha-value>)",
        "mys-card-dark": "rgb(var(--mys-card-dark)/<alpha-value>)",
        "mys-card-darker": "rgb(var(--mys-card-darker)/<alpha-value>)",
      },
      borderRadius: {
        mys: "40px",
      },
      boxShadow: {
        mys: "0px 0px 4px rgba(0, 0, 0, 0.02)",
        "mys-button": "0px 1px 2px rgba(16, 24, 40, 0.05)",
        "mys-notification": "0px 0px 20px rgba(29, 55, 87, 0.11)",
      },
      gradientColorStopPositions: {
        36: "36%",
      },
    },
  },
  plugins: [],
};
