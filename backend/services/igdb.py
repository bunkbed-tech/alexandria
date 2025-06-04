from datetime import datetime, timezone
from os import environ

from fastapi import APIRouter
from httpx import AsyncClient

from models import VideoGame


router = APIRouter()
params = {
    "client_id": environ["ALEXANDRIA_IGDB_CLIENT_ID"],
    "client_secret": environ["ALEXANDRIA_IGDB_CLIENT_SECRET"],
    "grant_type": "client_credentials",
}


@router.get("/search", response_model=list[VideoGame])
async def igdb_search(query: str):
    async with AsyncClient() as client:
        response = await client.post("https://id.twitch.tv/oauth2/token", params=params)
        response.raise_for_status()
        headers = {
            "Client-ID": params["client_id"],
            "Authorization": f"Bearer {response.json()['access_token']}",
        }

        response = await client.post(
            "https://api.igdb.com/v4/games",
            headers=headers,
            content=f"fields name, summary, cover, first_release_date; search \"{query}\"; limit 500;",
        )
        response.raise_for_status()
        games = response.json()

        game_ids = filter(None, [game["cover"]["id"] for game in games])
        response = await client.post(
            "https://api.igdb.com/v4/covers",
            headers=headers,
            content=f"fields url; where id = ({','.join(game_ids)}); limit 500;",
        )
        response.raise_for_status()
        covers = {cover["id"]: cover["url"] for cover in response.json()}

    return [
        VideoGame(
            api_id=game["id"],
            title=game["name"],
            description=game.get("summary"),
            year_published=(
                datetime.fromtimestamp(ts, tz=timezone.utc).year
                if (ts := game.get("first_release_date")) is not None
                else None
            ),
            thumbnail=f"https:{covers[game['cover']]}",
        )
        for game in games
    ]
