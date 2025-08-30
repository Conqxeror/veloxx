import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/veloxx/search',
    component: ComponentCreator('/veloxx/search', 'bdd'),
    exact: true
  },
  {
    path: '/veloxx/docs',
    component: ComponentCreator('/veloxx/docs', 'ec5'),
    routes: [
      {
        path: '/veloxx/docs',
        component: ComponentCreator('/veloxx/docs', '50e'),
        routes: [
          {
            path: '/veloxx/docs',
            component: ComponentCreator('/veloxx/docs', '83d'),
            routes: [
              {
                path: '/veloxx/docs/api/javascript',
                component: ComponentCreator('/veloxx/docs/api/javascript', '0ca'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/api/python',
                component: ComponentCreator('/veloxx/docs/api/python', 'f3c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/api/rust',
                component: ComponentCreator('/veloxx/docs/api/rust', 'cd0'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/getting-started/installation',
                component: ComponentCreator('/veloxx/docs/getting-started/installation', '9d2'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/getting-started/quick-start',
                component: ComponentCreator('/veloxx/docs/getting-started/quick-start', 'a00'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/intro',
                component: ComponentCreator('/veloxx/docs/intro', '884'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/performance-analysis',
                component: ComponentCreator('/veloxx/docs/performance-analysis', '827'),
                exact: true
              },
              {
                path: '/veloxx/docs/performance/benchmark-report',
                component: ComponentCreator('/veloxx/docs/performance/benchmark-report', '153'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/performance/benchmarks',
                component: ComponentCreator('/veloxx/docs/performance/benchmarks', '427'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/performance/competitive-benchmarks',
                component: ComponentCreator('/veloxx/docs/performance/competitive-benchmarks', 'ae1'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/performance/cross-language-analysis',
                component: ComponentCreator('/veloxx/docs/performance/cross-language-analysis', '707'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/performance/features',
                component: ComponentCreator('/veloxx/docs/performance/features', '7d3'),
                exact: true
              },
              {
                path: '/veloxx/docs/tutorials/async_json',
                component: ComponentCreator('/veloxx/docs/tutorials/async_json', '53c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/tutorials/customer_purchase_analysis',
                component: ComponentCreator('/veloxx/docs/tutorials/customer_purchase_analysis', 'd6c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/veloxx/docs/tutorials/general_tutorial',
                component: ComponentCreator('/veloxx/docs/tutorials/general_tutorial', '32e'),
                exact: true,
                sidebar: "tutorialSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/veloxx/',
    component: ComponentCreator('/veloxx/', '1dc'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
