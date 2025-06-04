from fastapi import APIRouter
from httpx import AsyncClient

from models import VisualNovel


router = APIRouter()


@router.get("/search", response_model=list[VisualNovel])
async def vndb_search(query: str):
    async with AsyncClient() as client:
        json = {
            "filters": ["search", "=", query],
            "fields": "title, description, released, image.thumbnail",
            "count": True,
            "results": 100,
        }
        response = await client.post("https://api.vndb.org/kana/vn", json=json)
        response.raise_for_status()
        return [
            VisualNovel(
                api_id=int(novel["id"][1:]),
                title=novel["title"],
                description=novel["description"],
                year_published=int(novel["released"][:4]),
                thumbnail=novel["image"]["thumbnail"],
            )
            for novel in response.json()["results"]
        ]
