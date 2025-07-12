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
    component: ComponentCreator('/veloxx/docs', '932'),
    routes: [
      {
        path: '/veloxx/docs',
        component: ComponentCreator('/veloxx/docs', '53f'),
        routes: [
          {
            path: '/veloxx/docs',
            component: ComponentCreator('/veloxx/docs', '939'),
            routes: [
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
                path: '/veloxx/docs/performance/benchmarks',
                component: ComponentCreator('/veloxx/docs/performance/benchmarks', '427'),
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
