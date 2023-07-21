from pydantic import BaseModel


class ImageBuildModel(BaseModel):
    language: str
    tag: str
    game_file_name: str
