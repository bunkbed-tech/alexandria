# alexandria

alexandria is your long lost media library

Track your book, movie, tv, and game backlogs. Maintain digital metadata libraries of your physical collections. Do it all without paid features or forced social media participation.

## Implementation

Create a desktop and mobile portal featuring free syncing between them. It would make sense then to use SQLite as the database and have the end-user store it on iCloud or Google Drive, similar to the way Obsidian allows for free syncing. The other option is to allow users to self-host the app in a container.

The desire is to develop the app using Rust's Tauri for desktop and native code for Android/iOS. Because Tauri let's you use any JavaScript framework, it should therefore be easy to develop with a web app in mind as well.

### Database APIs to Consider

- Open Movie Database
- Twitch
- Google Books
- Visual Novel Database
- Board Game Atlas
- Board Game Geek
- Discogs
- Anilist
- My Anime List

### Features to Consider

- Basic backlog tracking
- Owned physical media tracking
- Embeddable web widgets, e.g. "Currently Reading"

## Data Model

Here is our starting Mermaid diagram for a basic data model with just board games:

``` mermaid
erDiagram
    BOARD_GAME {
        int         id
        int         resource_id
        string[]    publishers
        string[]    designers
    }
    TV_SHOW {
        int         id
        int         resource_id
        int         season
        int         total_episodes
    }
    RESOURCE {
        int     id
        enum    type
        string  title
        string  description
        int     year_published
        bool    owned
        bool    want_to_own
        bool    want_to_try
        file    thumbnail
        file    image
        int     plays
        %% sum of all actions with end_datetime
    }
    ACTION {
        int         id
        int         resource_id
        datetime    start_datetime
        datetime    end_datetime
        string      notes
        enum        status
        %% doing, abandoned, finished
        int         progress
        %% only used if the resource is periodical
    }
    BOARD_GAME ||--|| RESOURCE : is
    TV_SHOW ||--|| RESOURCE : is
    RESOURCE ||--o{ ACTION : "is done by"
```
