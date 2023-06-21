from fastapi import APIRouter
from ..service import docker_image_service

router = APIRouter(
    prefix="/images",
    tags=["Images"],
    dependencies=[],
    responses={404: {"description": "Not found"}},
)


@router.head("/")
async def head():
    return


@router.post("/")
async def build_image(language: str, tag: str):
    tag = docker_image_service.build_image(language=language, tag=tag)
    if tag != "":
        docker_image_service.push_image(tag=tag)
        return "OK"
    return "not OK"
