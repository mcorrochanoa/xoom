// @ts-check
import { defineConfig } from 'astro/config'
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
            label: 'Guides',
            link: '/guides/',
            icon: 'open-book',
            items: ['guides/example']
          },
          {
            label: 'Reference',
            link: '/reference/',
            icon: 'information',
            items: ['reference/example']
          }
        ])
      ],
      social: [
        {
          icon: 'github',
          label: 'GitHub',
          href: 'https://github.com/mxuee/apt'
        }
      ]
    })
  ]
})
