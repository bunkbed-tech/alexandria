from pytest import mark

from models import BoardGame
from services.bgg import search_bgg, list_bgg_things

@mark.asyncio
async def test_search_bgg():
    resource_ids = await search_bgg("Cranium Cadoo")
    # BGG /search results order is non-deterministic
    resource_ids.sort()
    assert resource_ids == [6420, 14454]

@mark.asyncio
async def test_search_bgg_empty():
    assert await search_bgg("sdlkajlslkshlddk") == []

@mark.asyncio
async def test_list_bgg_things():
    resources = await list_bgg_things([6420, 14454])
    expected_resources = [
        BoardGame(
            api_id=6420,
            title="Cranium Cadoo",
            description="A version of Cranium &quot;scaled down&quot; for kids, although the game should still appeal to adults who like Cranium.  Here's the manufacturer's information:&#10;&#10;&quot;With a variety of hilarious activities, Cranium Cadoo gets kids thinking, creating, giggling, grinning, and laughing like crazy as they try to get four in a row to win. With so many different activities, there is something in Cranium Cadoo that will make every kid hoot and high-five. They might even discover a talent they never knew they had!&#10;&#10;And kids just love the cool Cranium Clay, funky tokens, and especially the Secret Decoder Mask. Whether kids love to act, puzzle, sketch, sculpt, or even crack secret codes, Cranium Cadoo has something for everyone&hellip;including you!&quot;&#10;&#10;",
            year_published=2001,
            thumbnail="https://cf.geekdo-images.com/hQI6W-7HwKty4c5yLFP-Aw__thumb/img/_IyE4nIyGh7_PVfGCarLoNmDMGc=/fit-in/200x150/filters:strip_icc()/pic3335930.jpg",
        ),
        BoardGame(
            api_id=14454,
            title="Cranium Cadoo Booster Box",
            description="Booster box with 300 new cards, Clay, secret decoder mask and drawing pad.&#10;&#10;Expands:&#10;&#10;    Cranium Cadoo&#10;&#10;&#10;",
            year_published=2001,
            thumbnail="https://cf.geekdo-images.com/jboSqbHm5jcQp7XJZPM-vw__thumb/img/v6dQ2IqIdGJIX19AVEZDSaQ5Nms=/fit-in/200x150/filters:strip_icc()/pic58689.jpg",
        ),
    ]
    assert resources == expected_resources

@mark.asyncio
async def test_list_bgg_things_invalid():
    assert await list_bgg_things([0]) == []
