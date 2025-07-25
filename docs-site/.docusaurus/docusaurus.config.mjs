/*
 * AUTOGENERATED - DON'T EDIT
 * Your edits in this file will be overwritten in the next build!
 * Modify the docusaurus.config.js file at your site's root instead.
 */
export default {
  "title": "Veloxx",
  "tagline": "Lightning-fast data processing for Rust, Python & JavaScript",
  "favicon": "img/favicon.ico",
  "url": "https://conqxeror.github.io",
  "baseUrl": "/veloxx/",
  "organizationName": "Conqxeror",
  "projectName": "veloxx",
  "onBrokenLinks": "throw",
  "onBrokenMarkdownLinks": "warn",
  "i18n": {
    "defaultLocale": "en",
    "locales": [
      "en"
    ],
    "path": "i18n",
    "localeConfigs": {}
  },
  "presets": [
    [
      "classic",
      {
        "docs": {
          "sidebarPath": "D:\\Projects\\Veloxx\\veloxx\\docs-site\\sidebars.js",
          "editUrl": "https://github.com/Conqxeror/veloxx/tree/main/docs-site/"
        },
        "blog": {
          "showReadingTime": true,
          "editUrl": "https://github.com/Conqxeror/veloxx/tree/main/docs-site/"
        },
        "theme": {
          "customCss": "D:\\Projects\\Veloxx\\veloxx\\docs-site\\src\\css\\custom.css"
        }
      }
    ]
  ],
  "themeConfig": {
    "image": "img/veloxx-social-card.jpg",
    "colorMode": {
      "defaultMode": "light",
      "disableSwitch": false,
      "respectPrefersColorScheme": true
    },
    "navbar": {
      "title": "Veloxx",
      "logo": {
        "alt": "Veloxx Logo",
        "src": "img/veloxx_logo.png"
      },
      "hideOnScroll": false,
      "items": [
        {
          "type": "docSidebar",
          "sidebarId": "tutorialSidebar",
          "position": "left",
          "label": "Docs"
        },
        {
          "type": "dropdown",
          "label": "API Reference",
          "position": "left",
          "items": [
            {
              "to": "/docs/api/rust",
              "label": "Rust API"
            },
            {
              "to": "/docs/api/python",
              "label": "Python API"
            },
            {
              "to": "/docs/api/javascript",
              "label": "JavaScript API"
            }
          ]
        },
        {
          "to": "/docs/performance/benchmarks",
          "label": "Benchmarks",
          "position": "left"
        },
        {
          "to": "/docs/intro",
          "label": "Blog",
          "position": "left"
        },
        {
          "href": "https://github.com/Conqxeror/veloxx",
          "label": "GitHub",
          "position": "right"
        },
        {
          "href": "https://crates.io/crates/veloxx",
          "label": "Crates.io",
          "position": "right"
        }
      ]
    },
    "footer": {
      "style": "light",
      "links": [
        {
          "title": "Docs",
          "items": [
            {
              "label": "Getting Started",
              "to": "/docs/getting-started/installation"
            },
            {
              "label": "Rust API",
              "to": "/docs/api/rust"
            },
            {
              "label": "Python API",
              "to": "/docs/api/python"
            },
            {
              "label": "JavaScript API",
              "to": "/docs/api/javascript"
            },
            {
              "label": "Examples",
              "to": "/docs/intro"
            }
          ]
        },
        {
          "title": "Community",
          "items": [
            {
              "label": "GitHub Discussions",
              "href": "https://github.com/Conqxeror/veloxx/discussions"
            },
            {
              "label": "Issues",
              "href": "https://github.com/Conqxeror/veloxx/issues"
            }
          ]
        },
        {
          "title": "More",
          "items": [
            {
              "label": "Blog",
              "to": "/docs/intro"
            },
            {
              "label": "GitHub",
              "href": "https://github.com/Conqxeror/veloxx"
            },
            {
              "label": "Crates.io",
              "href": "https://crates.io/crates/veloxx"
            }
          ]
        }
      ],
      "copyright": "Copyright © 2025 Veloxx. Built with Docusaurus."
    },
    "prism": {
      "theme": {
        "plain": {
          "color": "#393A34",
          "backgroundColor": "#f6f8fa"
        },
        "styles": [
          {
            "types": [
              "comment",
              "prolog",
              "doctype",
              "cdata"
            ],
            "style": {
              "color": "#999988",
              "fontStyle": "italic"
            }
          },
          {
            "types": [
              "namespace"
            ],
            "style": {
              "opacity": 0.7
            }
          },
          {
            "types": [
              "string",
              "attr-value"
            ],
            "style": {
              "color": "#e3116c"
            }
          },
          {
            "types": [
              "punctuation",
              "operator"
            ],
            "style": {
              "color": "#393A34"
            }
          },
          {
            "types": [
              "entity",
              "url",
              "symbol",
              "number",
              "boolean",
              "variable",
              "constant",
              "property",
              "regex",
              "inserted"
            ],
            "style": {
              "color": "#36acaa"
            }
          },
          {
            "types": [
              "atrule",
              "keyword",
              "attr-name",
              "selector"
            ],
            "style": {
              "color": "#00a4db"
            }
          },
          {
            "types": [
              "function",
              "deleted",
              "tag"
            ],
            "style": {
              "color": "#d73a49"
            }
          },
          {
            "types": [
              "function-variable"
            ],
            "style": {
              "color": "#6f42c1"
            }
          },
          {
            "types": [
              "tag",
              "selector",
              "keyword"
            ],
            "style": {
              "color": "#00009f"
            }
          }
        ]
      },
      "darkTheme": {
        "plain": {
          "color": "#9CDCFE",
          "backgroundColor": "#1E1E1E"
        },
        "styles": [
          {
            "types": [
              "prolog"
            ],
            "style": {
              "color": "rgb(0, 0, 128)"
            }
          },
          {
            "types": [
              "comment"
            ],
            "style": {
              "color": "rgb(106, 153, 85)"
            }
          },
          {
            "types": [
              "builtin",
              "changed",
              "keyword",
              "interpolation-punctuation"
            ],
            "style": {
              "color": "rgb(86, 156, 214)"
            }
          },
          {
            "types": [
              "number",
              "inserted"
            ],
            "style": {
              "color": "rgb(181, 206, 168)"
            }
          },
          {
            "types": [
              "constant"
            ],
            "style": {
              "color": "rgb(100, 102, 149)"
            }
          },
          {
            "types": [
              "attr-name",
              "variable"
            ],
            "style": {
              "color": "rgb(156, 220, 254)"
            }
          },
          {
            "types": [
              "deleted",
              "string",
              "attr-value",
              "template-punctuation"
            ],
            "style": {
              "color": "rgb(206, 145, 120)"
            }
          },
          {
            "types": [
              "selector"
            ],
            "style": {
              "color": "rgb(215, 186, 125)"
            }
          },
          {
            "types": [
              "tag"
            ],
            "style": {
              "color": "rgb(78, 201, 176)"
            }
          },
          {
            "types": [
              "tag"
            ],
            "languages": [
              "markup"
            ],
            "style": {
              "color": "rgb(86, 156, 214)"
            }
          },
          {
            "types": [
              "punctuation",
              "operator"
            ],
            "style": {
              "color": "rgb(212, 212, 212)"
            }
          },
          {
            "types": [
              "punctuation"
            ],
            "languages": [
              "markup"
            ],
            "style": {
              "color": "#808080"
            }
          },
          {
            "types": [
              "function"
            ],
            "style": {
              "color": "rgb(220, 220, 170)"
            }
          },
          {
            "types": [
              "class-name"
            ],
            "style": {
              "color": "rgb(78, 201, 176)"
            }
          },
          {
            "types": [
              "char"
            ],
            "style": {
              "color": "rgb(209, 105, 105)"
            }
          }
        ]
      },
      "additionalLanguages": [
        "rust",
        "python",
        "javascript",
        "toml",
        "bash",
        "json"
      ],
      "magicComments": [
        {
          "className": "theme-code-block-highlighted-line",
          "line": "highlight-next-line",
          "block": {
            "start": "highlight-start",
            "end": "highlight-end"
          }
        }
      ]
    },
    "algolia": {
      "appId": "YOUR_APP_ID",
      "apiKey": "YOUR_SEARCH_API_KEY",
      "indexName": "veloxx",
      "contextualSearch": true,
      "externalUrlRegex": "external\\.com|domain\\.com",
      "replaceSearchResultPathname": {
        "from": "/docs/",
        "to": "/"
      },
      "searchParameters": {},
      "searchPagePath": "search"
    },
    "docs": {
      "versionPersistence": "localStorage",
      "sidebar": {
        "hideable": false,
        "autoCollapseCategories": false
      }
    },
    "blog": {
      "sidebar": {
        "groupByYear": true
      }
    },
    "metadata": [],
    "tableOfContents": {
      "minHeadingLevel": 2,
      "maxHeadingLevel": 3
    }
  },
  "baseUrlIssueBanner": true,
  "future": {
    "v4": {
      "removeLegacyPostBuildHeadAttribute": false,
      "useCssCascadeLayers": false
    },
    "experimental_faster": {
      "swcJsLoader": false,
      "swcJsMinimizer": false,
      "swcHtmlMinimizer": false,
      "lightningCssMinimizer": false,
      "mdxCrossCompilerCache": false,
      "rspackBundler": false,
      "rspackPersistentCache": false,
      "ssgWorkerThreads": false
    },
    "experimental_storage": {
      "type": "localStorage",
      "namespace": false
    },
    "experimental_router": "browser"
  },
  "onBrokenAnchors": "warn",
  "onDuplicateRoutes": "warn",
  "staticDirectories": [
    "static"
  ],
  "customFields": {},
  "plugins": [],
  "themes": [],
  "scripts": [],
  "headTags": [],
  "stylesheets": [],
  "clientModules": [],
  "titleDelimiter": "|",
  "noIndex": false,
  "markdown": {
    "format": "mdx",
    "mermaid": false,
    "mdx1Compat": {
      "comments": true,
      "admonitions": true,
      "headingIds": true
    },
    "anchors": {
      "maintainCase": false
    }
  }
};
