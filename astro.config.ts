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
            label: 'Databases',
            link: '/databases/',
            id: 'databases',
            icon: 'seti:db',
            items: [
              { label: 'SQL', autogenerate: { directory: 'databases/sql' } },
              {
                label: 'NoSQL',
                autogenerate: { directory: 'databases/nosql' }
              },
              { label: 'ORM', autogenerate: { directory: 'databases/orm' } }
            ]
          },
          {
            label: 'Frameworks',
            link: '/frameworks/',
            id: 'frameworks',
            icon: 'seti:hex',
            items: []
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
      }
    })
  ]
})
