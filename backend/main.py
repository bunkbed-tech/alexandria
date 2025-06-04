from fastapi import FastAPI, Depends, HTTPException
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select

import models
import schemas
import database

app = FastAPI()

async def get_session() -> AsyncSession:
    async with database.SessionLocal() as session:
        yield session

from fastapi import FastAPI
from services import bgg, resource, vndb, anilist, igdb, search

app = FastAPI()

app.include_router(bgg.router, prefix="/bgg")
app.include_router(resource.router, prefix="/resource")
app.include_router(vndb.router, prefix="/vndb")
app.include_router(anilist.router, prefix="/anilist")
app.include_router(igdb.router, prefix="/igdb")
app.include_router(search.router, prefix="/search")

@app.on_event("startup")
async def on_startup():
    await database.init_db()

@app.get("/books", response_model=list[schemas.BookOut])
async def list_books(session: AsyncSession = Depends(get_session)):
    result = await session.execute(select(models.Book))
    return result.scalars().all()

@app.post("/books", response_model=schemas.BookOut)
async def create_book(book: schemas.BookCreate, session: AsyncSession = Depends(get_session)):
    new_book = models.Book(**book.dict())
    session.add(new_book)
    await session.commit()
    await session.refresh(new_book)
    return new_book

@app.put("/books/{book_id}", response_model=schemas.BookOut)
async def update_book(book_id: int, book: schemas.BookCreate, session: AsyncSession = Depends(get_session)):
    result = await session.get(models.Book, book_id)
    if not result:
        raise HTTPException(status_code=404, detail="Book not found")
    result.title = book.title
    result.author = book.author
    await session.commit()
    await session.refresh(result)
    return result

@app.delete("/books/{book_id}")
async def delete_book(book_id: int, session: AsyncSession = Depends(get_session)):
    book = await session.get(models.Book, book_id)
    if not book:
        raise HTTPException(status_code=404, detail="Book not found")
    await session.delete(book)
    await session.commit()
    return {"ok": True}
