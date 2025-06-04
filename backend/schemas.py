from pydantic import BaseModel

class BookCreate(BaseModel):
    title: str
    author: str

class BookOut(BookCreate):
    id: int

    class Config:
        orm_mode = True
