from pydantic import BaseModel


class ContainerExecModel(BaseModel):
    language: str
    commands: list[str]
