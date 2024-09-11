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
3. [ ] Add more APIs
4. [ ] Package standalone release with database
5. [ ] Make combobox options dynamic

## v2 Todos

1. [ ] Server-client architecture
2. [ ] Self-hosted templates or packages (k8s, docker, etc., NIX)
3. [ ] Mobile app
4. [ ] data export

## Frontend Styling

1. [x] minimum window size
2. [x] white text for everything with orange background
3. [ ] padding between pagination and title and grid
4. [ ] dynamic header
5. [ ] delay filter activation (must be clicked!)
6. [ ] labels for filters
7. [ ] proper form for filter dialog
8. [ ] increments for slider and labels for knobs
9. [ ] same highlight on hover mechanism for resource card buttons and pagination
10. [ ] thin dividers on the sides of pagination
11. [ ] FIX THE GRID
12. [ ] carousel view
13. [ ] items per page setting
14. [ ] elongate placeholder thumbnail to full size
15. [ ] fix infinite scroll of titles
16. [ ] blow up and gray out thumbnail as new background
17. [ ] bug where wrong items in tracked are removed
18. [ ] logos
19. [ ] function loading placeholder (spinner, progress bar)
20. [ ] undo toast
21. [ ] untrack confirmation dialog
22. [ ] get slider to work again

## APIs

1. [ ] twitch/amazon api for video games
2. [ ] google books api
3. [ ] discogs
4. [ ] myanimelist
5. [ ] vndb
