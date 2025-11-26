// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer').themes.github;
const darkCodeTheme = require('prism-react-renderer').themes.vsDark;

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'VELOXX',
  tagline: 'LIGHTNING-FAST DATA PROCESSING FOR RUST, PYTHON & JAVASCRIPT',
  favicon: 'img/favicon.ico',

  url: 'https://conqxeror.github.io',
  baseUrl: '/veloxx/',

  organizationName: 'conqxeror',
  projectName: 'veloxx',

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

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
          editUrl: 'https://github.com/conqxeror/veloxx/tree/main/docs-site/',
        },
        blog: {
          showReadingTime: true,
          editUrl: 'https://github.com/conqxeror/veloxx/tree/main/docs-site/',
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
      image: 'img/veloxx-social-card.jpg',
      colorMode: {
        defaultMode: 'light',
        disableSwitch: false,
        respectPrefersColorScheme: true,
      },
      navbar: {
        title: 'VELOXX',
        logo: {
          alt: 'Veloxx Logo',
          src: 'img/veloxx_logo.png',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'DOCS',
          },
          {
            type: 'dropdown',
            label: 'API',
            position: 'left',
            items: [
              {
                to: '/docs/api/rust',
                label: 'RUST',
              },
              {
                to: '/docs/api/python',
                label: 'PYTHON',
              },
              {
                to: '/docs/api/javascript',
                label: 'JAVASCRIPT',
              },
            ],
          },
          {
            to: '/docs/performance/benchmarks',
            label: 'BENCHMARKS',
            position: 'left',
          },
          {
            href: 'https://github.com/conqxeror/veloxx',
            label: 'GITHUB',
            position: 'right',
          },
          {
            href: 'https://crates.io/crates/veloxx',
            label: 'CRATES.IO',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'light',
        links: [
          {
            title: 'DOCS',
            items: [
              {
                label: 'GETTING STARTED',
                to: '/docs/getting-started/installation',
              },
              {
                label: 'RUST API',
                to: '/docs/api/rust',
              },
              {
                label: 'PYTHON API',
                to: '/docs/api/python',
              },
              {
                label: 'JAVASCRIPT API',
                to: '/docs/api/javascript',
              },
            ],
          },
          {
            title: 'COMMUNITY',
            items: [
              {
                label: 'GITHUB DISCUSSIONS',
                href: 'https://github.com/conqxeror/veloxx/discussions',
              },
              {
                label: 'ISSUES',
                href: 'https://github.com/conqxeror/veloxx/issues',
              },
            ],
          },
          {
            title: 'MORE',
            items: [
              {
                label: 'BLOG',
                to: '/docs/intro',
              },
              {
                label: 'GITHUB',
                href: 'https://github.com/conqxeror/veloxx',
              },
            ],
          },
        ],
        copyright: `COPYRIGHT © ${new Date().getFullYear()} VELOXX. BUILT WITH DOCUSAURUS.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['rust', 'python', 'javascript', 'toml', 'bash', 'json'],
      },
    }),
};

module.exports = config;