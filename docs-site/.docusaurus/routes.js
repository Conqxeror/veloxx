import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/veloxx/__docusaurus/debug',
    component: ComponentCreator('/veloxx/__docusaurus/debug', '93a'),
    exact: true
  },
  {
    path: '/veloxx/__docusaurus/debug/config',
    component: ComponentCreator('/veloxx/__docusaurus/debug/config', '76d'),
    exact: true
  },
  {
    path: '/veloxx/__docusaurus/debug/content',
    component: ComponentCreator('/veloxx/__docusaurus/debug/content', '3dc'),
    exact: true
  },
  {
    path: '/veloxx/__docusaurus/debug/globalData',
    component: ComponentCreator('/veloxx/__docusaurus/debug/globalData', '860'),
    exact: true
  },
  {
    path: '/veloxx/__docusaurus/debug/metadata',
    component: ComponentCreator('/veloxx/__docusaurus/debug/metadata', '762'),
    exact: true
  },
  {
    path: '/veloxx/__docusaurus/debug/registry',
    component: ComponentCreator('/veloxx/__docusaurus/debug/registry', '322'),
    exact: true
  },
  {
    path: '/veloxx/__docusaurus/debug/routes',
    component: ComponentCreator('/veloxx/__docusaurus/debug/routes', 'afa'),
    exact: true
  },
  {
    path: '/veloxx/search',
    component: ComponentCreator('/veloxx/search', 'bdd'),
    exact: true
  },
  {
    path: '/veloxx/docs',
    component: ComponentCreator('/veloxx/docs', '8ef'),
    routes: [
      {
        path: '/veloxx/docs',
        component: ComponentCreator('/veloxx/docs', 'd9f'),
        routes: [
          {
            path: '/veloxx/docs',
            component: ComponentCreator('/veloxx/docs', '148'),
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
