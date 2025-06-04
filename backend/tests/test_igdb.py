from pytest import mark

from models import VideoGame
from services.igdb import igdb_search


@mark.asyncio
async def test_search_igdb():
    assert await igdb_search("fretless") == [
        VideoGame(
            api_id=252794,
            title="Fretless",
            year_published=2025,
            description="In this turned-based RPG, wield powerful legendary instruments, gather mighty riff attacks and save the land from Rick Riffson\u{0027}s devilish goons and musical hybrid monsters!",
            thumbnail="https://images.igdb.com/igdb/image/upload/t_thumb/co6lw2.jpg",
        ),
    ]
