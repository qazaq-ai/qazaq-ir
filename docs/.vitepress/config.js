import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Qazaq IR",
  description: "The First Agglutinative Intermediate Representation for Deterministic LLM Code Generation",
  base: '/qazaq-ir/',

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/shanraq_neuron.svg' }]
  ],

  // Minimalist default theme configuration
  themeConfig: {
    logo: '/shanraq_neuron.svg',

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Documentation', link: '/origin' },
      { text: 'Whitepaper', link: 'https://github.com/qazaq-ai/qazaq-ir/blob/main/WHITEPAPER.md' },
    ],

    sidebar: [
      {
        text: 'The Origin (История Идеи)',
        items: [
          { text: 'The Analytical Bottleneck', link: '/origin' }
        ]
      },
      {
        text: 'Architecture (Архитектура)',
        items: [
          { text: 'Linear Tokens & O(1) Design', link: '/architecture' }
        ]
      },
      {
        text: 'Core Engine (Реализация)',
        items: [
          { text: 'O(1) Bitwise Validation', link: '/core-engine' },
          { text: 'Semantic Router', link: '/semantic-router' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/qazaq-ai/qazaq-ir' }
    ],

    footer: {
      message: 'Released under the Dual License (CC BY-ND 4.0 & BSL 1.1).',
      copyright: 'Copyright © 2026 Daulet Baimurza'
    }
  }
})
