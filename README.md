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
3. [x] padding between pagination and title and grid
4. [x] dynamic header
5. [x] labels for filters
6. [x] increments for slider and labels for knobs
7. [ ] filter knobs should be fine tuned to allow for arbitrary filtering between years instead of binary
8. [x] same highlight on hover mechanism for resource card buttons and pagination
9. [ ] thin dividers on the sides of pagination
10. [ ] FIX THE GRID
11. [ ] carousel view
12. [ ] items per page setting
13. [ ] elongate placeholder thumbnail to full size
14. [ ] fix infinite scroll of titles
15. [ ] blow up and gray out thumbnail as new background
16. [ ] bug where wrong items in tracked are removed
17. [ ] logos
18. [ ] function loading placeholder (spinner, progress bar)
19. [ ] undo toast
20. [ ] untrack confirmation dialog
21. [ ] get slider to work again

## APIs

1. [ ] twitch/amazon api for video games
2. [ ] google books api
3. [ ] discogs
4. [ ] myanimelist
5. [ ] vndb
