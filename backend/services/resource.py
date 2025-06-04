from fastapi import APIRouter

from models import AnimeMovie, AnimeTVShow, Manga, LightNovel, VisualNovel, VideoGame, BoardGame


Resource = AnimeMovie | AnimeTVShow | Manga | LightNovel | VisualNovel | VideoGame | BoardGame
router = APIRouter()


@router.get("/", response_model=list[Resource])
async def resource_list():
    pass
