from asyncio import gather

from fastapi import APIRouter, Query, HTTPException
from boardgamegeek import BGGClient

from models import BoardGame

router = APIRouter()
bgg = BGGClient()

@router.get("/search", response_model=list[int])
async def search_bgg(query: str = Query(..., min_length=1)):
    try:
        games = bgg.search(query)
    except Exception as e:
        raise HTTPException(status_code=500, detail="BGG initial search failed") from e

    return [game["id"] for game in games]

@router.get("/things", response_model=list[BoardGame])
async def bgg_things_list(ids: list[int]):
    try:
        games = bgg.games_list(ids)
    except Exception as e:
        raise HTTPException(status_code=500, detail="BGG things list failed") from e
    return [
        BoardGame(
            id=None,
            title=game["name"],
            description=game["description"],
            year_published=game["yearpublished"],
            thumbnail=game["thumbnail"],
            api_id=game["id"],
        )
        for game in games
    ]

@router.get("/things/search", response_model=list[BoardGame])
async def bgg_things_search(
    query: str = Query(..., min_length=1),
    limit: int = 100,
):
    # TODO propagate all the errors and resources as a tuple
    # limit the number of resources so as not to take too long
    game_ids = await search_bgg(query)[:limit]
    # BGG /thing API has a limit of 20 IDs, so we chunk the IDs
    chunk_size = 20
    chunked_ids = [game_ids[i:i + chunk_size] for i in range(0, len(game_ids), chunk_size)]
    chunked_games = await gather(*[bgg_things_list(ids_chunk) for ids_chunk in chunked_ids])
    games = [game for games_chunk in chunked_games for game in games_chunk]
    # BGG doesn't return things in a reliable order, so we sort by api_id
    return sorted(games, key=lambda game: game["api_id"])
