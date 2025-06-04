from enum import Enum

from fastapi import APIRouter
from httpx import AsyncClient

from models import AnimeMovie, AnimeTVShow, Manga, LightNovel


class MediaType(str, Enum):
    ANIME = "ANIME"
    MANGA = "MANGA"

class MediaFormat(str, Enum):
    TV = "TV"
    MOVIE = "MOVIE"
    MANGA = "MANGA"
    NOVEL = "NOVEL"

AnilistResource = AnimeMovie | AnimeTVShow | Manga | LightNovel


FORMAT_TO_TYPE = {
    MediaFormat.TV: MediaType.ANIME,
    MediaFormat.MOVIE: MediaType.ANIME,
    MediaFormat.MANGA: MediaType.MANGA,
    MediaFormat.NOVEL: MediaType.MANGA,
}
FORMAT_TO_CLASS = {
    "TV": AnimeTVShow,
    "MOVIE": AnimeMovie,
    "MANGA": Manga,
    "NOVEL": LightNovel,
}
QUERY = """
query ($search: String!, $type: MediaType, $formats: [MediaFormat!]!) {
  Page {
    media (search: $search, type: $type, format_in: $formats) {
      id
      description
      startDate {
        year
      }
      title {
        english
      }
      coverImage {
        medium
      }
      format
    }
  }
"""


router = APIRouter()


@router.get("/search", response_model=list[AnilistResource])
async def anilist_search(search: str, media_format: MediaFormat | list[MediaFormat]):
    payload = {
        "query": QUERY,
        "variables": {
            "search": search,
            "formats": list(media_format),
        },
    }
    media_types = list({FORMAT_TO_TYPE[format] for format in payload["variables"]["formats"]})
    if len(media_types) == 1:
        payload["variables"]["type"] = media_types[0]

    async with AsyncClient() as client:
        response = await client.post("https://graphql.anilist.co", json=payload)
    response.raise_for_status()

    return [
        FORMAT_TO_CLASS[item["format"]](
            title=item["title"]["english"],
            description=item.get("description"),
            year_published=item["startDate"].get("year"),
            thumbnail=item["coverImage"].get("medium"),
            api_id=item["id"],
        )
        for item in response.json()["data"]["Page"]["media"]
    ]
