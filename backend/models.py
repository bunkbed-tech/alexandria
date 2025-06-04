from sqlalchemy.orm import DeclarativeBase, Mapped, declared_attr, mapped_column, relationship
from sqlalchemy import ForeignKey, Integer, String, UniqueConstraint

class Base(DeclarativeBase):
    pass

class Resource(Base):
    __tablename__ = "resource"

    id: Mapped[int] = mapped_column(Integer, primary_key=True, index=True)
    title: Mapped[str] = mapped_column(String, nullable=False)
    description: Mapped[str | None] = mapped_column(String)
    year_published: Mapped[int | None] = mapped_column(Integer)
    thumbnail: Mapped[str | None] = mapped_column(String)
    api_id: Mapped[int] = mapped_column(Integer, nullable=False)

    taggings: list["Tagging"] = relationship("Tagging", back_populates="resource")
    tags: list["Tag"] = relationship("Tag", secondary="taggings", back_populates="resources")

    def __str__(self):
        return f"{self.title} ({self.id}) [{self.api_id}]"

class Tag(Base):
    __tablename__ = "tag"

    name: Mapped[str] = mapped_column(String, nullable=False, unique=True)

    taggings: list["Tagging"] = relationship("Tagging", back_populates="tag")
    resources: list[Resource] = relationship(Resource, secondary="taggings", back_populates="tags")

    def __str__(self):
        return self.name

class Tagging(Base):
    __tablename__ = "tagging"
    __table_args__ = (
        UniqueConstraint("tag_id", "resource_id", name="uq_tag_resource"),
    )

    id: int = mapped_column(Integer, primary_key=True)
    tag_id: int = mapped_column(ForeignKey("tag.id"), nullable=False)
    resource_id: int = mapped_column(ForeignKey("resource.id"), nullable=False)

    tag: Tag = relationship(Tag, back_populates="taggings")
    resource: Resource = relationship(Resource, back_populates="taggings")

    def __str__(self):
        return f"{self.resource} <-> {self.tag}"

class ResourceMixin:
    @declared_attr
    def id(cls) -> Mapped[int]:
        return mapped_column(ForeignKey("resource.id"), nullable=False)

    @declared_attr
    def resource(cls) -> Resource:
        return relationship(Resource, uselist=False, back_populates=cls.__tablename__)

    def __str__(self):
        return f"{self.__class__.__name__}: {self.resource}"

class BoardGame(Base, ResourceMixin):
    __tablename__ = "board_game"

class VideoGame(Base, ResourceMixin):
    __tablename__ = "video_game"

class AnimeMovie(Base, ResourceMixin):
    __tablename__ = "anime_movie"

class AnimeTVShow(Base, ResourceMixin):
    __tablename__ = "anime_tv_show"

class Manga(Base, ResourceMixin):
    __tablename__ = "manga"

class LightNovel(Base, ResourceMixin):
    __tablename__ = "light_novel"

class VisualNovel(Base, ResourceMixin):
    __tablename__ = "visual_novel"
