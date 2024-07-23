# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/main/packages/create-svelte).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npm create svelte@latest

# create a new project in my-app
npm create svelte@latest my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

## Todo

1. [ ] Retain state in app between navigation
2. [ ] Fix up list creation buttons
3. [ ] make ResourceCard look good (hover, grid spacing, photo sizes)
4. [ ] Add more APIs
5. [ ] Sorting BGG API results by bgg_id
6. [ ] Filtering, sorting, search on list pages
7. [ ] Package standalone release with database
8. [ ] Make combobox options dynamic

## v2 Todos

1. [ ] Server-client architecture
2. [ ] Self-hosted templates or packages (k8s, docker, etc., NIX)
3. [ ] Mobile app
