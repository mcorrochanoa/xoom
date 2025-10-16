// @ts-check
import { defineConfig, fontProviders } from 'astro/config'
import starlight from '@astrojs/starlight'
import starlightSidebarTopics from 'starlight-sidebar-topics'

// https://astro.build/config
export default defineConfig({
  site: 'https://apt.mxuee.com',
  integrations: [
    starlight({
      title: 'APT',
      disable404Route: true,
      plugins: [
        starlightSidebarTopics([
          {
            label: 'Databases',
            link: '/databases/',
            id: 'databases',
            icon: 'seti:db',
            items: [
              {
                label: 'NoSQL',
                autogenerate: { directory: 'databases/nosql' }
              }
            ]
          }
        ])
      ],
      social: [
        {
          icon: 'github',
          label: 'GitHub',
          href: 'https://github.com/mxuee/apt'
        }
      ],
      editLink: {
        baseUrl: 'https://github.com/mxuee/apt/edit/main/'
      },
      components: {
        Head: './src/components/Head.astro'
      },
      customCss: ['./src/styles/custom.css']
    })
  ],
  experimental: {
    fonts: [
      {
        cssVariable: '--font-inter',
        name: 'Inter',
        provider: fontProviders.google()
      }
    ]
  }
})
