from pytest import mark

from models import AnimeMovie, AnimeTVShow, Manga, LightNovel
from services.anilist import search_anilist, MediaFormat

@mark.asyncio
async def test_search_anilist_movie():
    assert await search_anilist("Paprika", MediaFormat.Movie) == [
        AnimeMovie(
            api_id=1943,
            title="Paprika",
            description="Prepare to enter the realm of fantasy and imagination where reality and dreams collide in a kaleidoscopic mindscape of sheer visual genius. The magical tale centers on a revolutionary machine that allows scientists to enter and record a subject's dream. After being stolen, a fearless detective and brilliant therapist join forces to recover the device before it falls into the hands of a dream terrorist.<br>\n<br>\n(Source: Sony Pictures Home Entertainment)<br>\n<br>\n<i>Note: The film received an early premiere at the 63rd Venice International Film Festival on September 2, 2006.</i>",
            year_published=2006,
            thumbnail="https://s4.anilist.co/file/anilistcdn/media/anime/cover/small/b1943-TBNhMVA9VwdI.png",
        ),
    ]

@mark.asyncio
async def test_search_anilist_tv():
    assert await search_anilist("Buddy Daddies", MediaFormat.TV) == [
        AnimeTVShow(
            api_id=155907,
            title="Buddy Daddies",
            description="Assassins Kazuki Kurusu and Rei Suwa meet Miri, a girl looking for her father on Christmas Day. Kazuki, Rei, and Miri unexpectedly end up living together.<br>\n<br>\nFollows Kazuki Kurusu, a criminal contractor/coordinator who lives with his best friend, Rei Suwa, a professional assassin who has been raised from childhood to be a contract killer. Kazuki is outgoing and loves gambling and women, while Rei is a man of few words who spends his off time playing video games. One day, the two buddies end up caring for Miri Unasaka, a four year old girl whose father is a mafia boss, after Miri accidentally wanders into a firefight in a hotel while looking for her father.",
            year_published=2023,
            thumbnail="https://s4.anilist.co/file/anilistcdn/media/anime/cover/small/bx155907-gR7aRwVHwrjc.jpg",
        ),
    ]

@mark.asyncio
async def test_search_anilist_novel():
    assert await search_anilist("All You Need Is Kill", MediaFormat.NOVEL) == [
        LightNovel(
            api_id=48511,
            title="All You Need is Kill",
            description="When the alien Mimics invade, Keiji Kiriya is just one of many recruits shoved into a suit of battle armor called a Jacket and sent out to kill. Keiji dies on the battlefield, only to be reborn each morning to fight and die again and again. On his 158th iteration, he gets a message from a mysterious ally--the female soldier known as the Full Metal Bitch. Is she the key to Keiji's escape or his final death?\n<br><br>\n(Source: Viz Media)",
            year_published=2004,
            thumbnail="https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx48511-HJpsLXWtjHTz.jpg",
        ),
    ]

@mark.asyncio
async def test_search_anilist_manga():
    assert await search_anilist("All You Need Is Kill", MediaFormat.MANGA) == [
        Manga(
            api_id=85215,
            title="All You Need Is Kill",
            description="When the alien Mimics invade, Keiji Kiriya is just one of many recruits shoved into a suit of battle armor called a Jacket and sent out to kill. Keiji dies on the battlefield, only to be reborn each morning to fight and die again and again. On his 158th iteration, he gets a message from a mysterious ally--the female soldier known as the Full Metal Bitch. Is she the key to Keiji's escape or his final death?\n<br><br>\n(Source: Viz Media)",
            year_published=2014,
            thumbnail="https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx85215-oHqG7fkrpas9.png",
        ),
    ]

@mark.asyncio
async def test_search_anilist_multiple_formats():
    assert await search_anilist("All You Need Is Kill", [MediaFormat.MANGA, MediaFormat.NOVEL]) == [
        Manga(
            api_id=85215,
            title="All You Need Is Kill",
            description="When the alien Mimics invade, Keiji Kiriya is just one of many recruits shoved into a suit of battle armor called a Jacket and sent out to kill. Keiji dies on the battlefield, only to be reborn each morning to fight and die again and again. On his 158th iteration, he gets a message from a mysterious ally--the female soldier known as the Full Metal Bitch. Is she the key to Keiji's escape or his final death?\n<br><br>\n(Source: Viz Media)",
            year_published=2014,
            thumbnail="https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx85215-oHqG7fkrpas9.png",
        ),
        LightNovel(
            api_id=48511,
            title="All You Need is Kill",
            description="When the alien Mimics invade, Keiji Kiriya is just one of many recruits shoved into a suit of battle armor called a Jacket and sent out to kill. Keiji dies on the battlefield, only to be reborn each morning to fight and die again and again. On his 158th iteration, he gets a message from a mysterious ally--the female soldier known as the Full Metal Bitch. Is she the key to Keiji's escape or his final death?\n<br><br>\n(Source: Viz Media)",
            year_published=2004,
            thumbnail="https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx48511-HJpsLXWtjHTz.jpg",
        ),
    ]

@mark.asyncio
async def test_search_anilist_multiple_types():
    assert await search_anilist("Samurai Champloo", [MediaFormat.TV, MediaFormat.MANGA]) == [
        AnimeTVShow(
            api_id=205,
            title="Samurai Champloo",
            description="Let's break it down. Mugen's a reckless sword-slinger with a style that's more b-boy than Shaolin. He's got a nasty streak that makes people want to stick a knife in his throat. Then there's Jin, a deadbeat ronin who speaks softly but carries a big blade. He runs game old-school style, but he can make your blood spray with the quickness. When these roughnecks bring the ruckus, it ain't good for anybody, especially them. Enter Fuu, the ditzy waitress who springs her new friends from a deadly jam. All she wants in return is help solving a riddle from her past. She and the boys are tracking the scent, but there's 99 ways to die between them and the sunflower samurai.<br>\n<br>\n(Source: Funimation)",
            year_published=2004,
            thumbnail="https://s4.anilist.co/file/anilistcdn/media/anime/cover/small/bx205-xxonQKyJtVcw.png",
        ),
        Manga(
            api_id=30512,
            title="Samurai Champloo",
            description="Mugen is a rough-around-the-edges mercenary with a killer technique and nothing left to lose. Jin is a disciplined samurai who's as deadly as he is reserved. Fuu is a young waitress with a good heart and a resourcefulness that emerges when you least expect it. These three unlikely companions are about to begin a journey that will change all of their lives.<br><br>\nIt's a dangerous quest for a mysterious samurai that will see our squabbling group of heroes get into and out of trouble more times than they can count (which admittedly, isn't very high). From the cynical gentility of the nobles to the backstabbing of the Japanese underworld, Mugen, Jin and Fuu will face threats from without and within as they hurl insults and throwing stars alike. Ancient Japan is about to get a lethal dose of street justice -- Champloo style. And it will never be the same.<br><br>\n(Source: Tokyopop)",
            year_published=2004,
            thumbnail="https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx30512-L7FWQ9Dj6dHj.png",
        ),
    ]
