from pytest import mark

from models import VisualNovel
from services.vndb import vndb_search


@mark.asyncio
async def test_search_vndb():
    assert await vndb_search("steins divergence") == [
        VisualNovel(
            api_id=15695,
            title="Steins;Fate Divergence",
            year_published=2014,
            description=None,
            thumbnail="https://t.vndb.org/cv/83/21383.jpg",
        ),
    ]
