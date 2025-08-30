/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  tutorialSidebar: [
    'intro',
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'getting-started/installation',
        'getting-started/quick-start',
      ],
    },
    {
      type: 'category',
      label: 'API Reference',
      items: [
        'api/rust',
        'api/python',
        'api/javascript',
      ],
    },
    {
      type: 'category',
      label: 'Tutorials & Guides',
      items: [
        'tutorials/async_json',
        'tutorials/customer_purchase_analysis',
        'tutorials/general_tutorial',
      ],
    },
    {
      type: 'category',
      label: 'Examples',
      items: [
        {
          type: 'link',
          label: 'Data Processing Examples (GitHub)',
          href: 'https://github.com/conqxeror/veloxx/tree/main/examples',
        },
      ],
    },
    {
      type: 'category',
      label: 'Performance',
      items: [
        'performance/competitive-benchmarks',
        'performance/cross-language-analysis',
        'performance/benchmarks',
        'performance/benchmark-report',
      ],
    },
  ],
};

module.exports = sidebars;