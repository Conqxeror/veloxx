// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer').themes.github;
const darkCodeTheme = require('prism-react-renderer').themes.dracula;

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Veloxx',
  tagline: 'Lightning-fast data processing for Rust, Python & JavaScript',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://conqxeror.github.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/veloxx/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'Conqxeror', // Usually your GitHub org/user name.
  projectName: 'veloxx', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/Conqxeror/veloxx/tree/main/docs-site/',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/Conqxeror/veloxx/tree/main/docs-site/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/veloxx-social-card.jpg',
      navbar: {
        title: 'Veloxx',
        logo: {
          alt: 'Veloxx Logo',
          src: 'img/veloxx_logo.png',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'Docs',
          },
          {
            to: '/docs/api/rust',
            label: 'API Reference',
            position: 'left',
          },
          {
            to: '/docs/performance/benchmarks',
            label: 'Benchmarks',
            position: 'left',
          },
          {to: '/docs/intro', label: 'Blog', position: 'left'},
          {
            href: 'https://github.com/Conqxeror/veloxx',
            label: 'GitHub',
            position: 'right',
          },
          {
            href: 'https://crates.io/crates/veloxx',
            label: 'Crates.io',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Getting Started',
                to: '/docs/getting-started/installation',
              },
              {
                label: 'API Reference',
                to: '/docs/api/rust',
              },
              {
                label: 'Examples',
                to: '/docs/intro',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'GitHub Discussions',
                href: 'https://github.com/Conqxeror/veloxx/discussions',
              },
              {
                label: 'Issues',
                href: 'https://github.com/Conqxeror/veloxx/issues',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'Blog',
                to: '/docs/intro',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/Conqxeror/veloxx',
              },
              {
                label: 'Crates.io',
                href: 'https://crates.io/crates/veloxx',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Veloxx. Built with Docusaurus.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['rust', 'python', 'javascript', 'toml'],
      },
      algolia: {
        // The application ID provided by Algolia
        appId: 'YOUR_APP_ID',
        // Public API key: it is safe to commit it
        apiKey: 'YOUR_SEARCH_API_KEY',
        indexName: 'veloxx',
        // Optional: see doc section below
        contextualSearch: true,
        // Optional: Specify domains where the navigation should occur through window.location instead on history.push. Useful when our Algolia config crawls multiple documentation sites and we want to navigate with window.location.href to them.
        externalUrlRegex: 'external\\.com|domain\\.com',
        // Optional: Replace parts of the item URLs from Algolia. Useful when using the same search index for multiple deployments using a different baseUrl. You can use regexp or string in the `from` param. For example: localhost:3000 vs myCompany.github.io/myProject/
        replaceSearchResultPathname: {
          from: '/docs/', // or as RegExp: /\/docs\//
          to: '/',
        },
        // Optional: Algolia search parameters
        searchParameters: {},
        // Optional: path for search page that enabled by default (`false` to disable it)
        searchPagePath: 'search',
      },
    }),
};

module.exports = config;