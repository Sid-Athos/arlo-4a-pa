from fastapi import APIRouter, status
from fastapi.responses import JSONResponse
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
        if docker_image_service.push_image(tag=tag) == "OK":
            return JSONResponse(status_code=status.HTTP_201_CREATED, content={"tag": tag})
    return JSONResponse(status_code=status.HTTP_400_BAD_REQUEST, content="")


@router.delete("/")
async def delete_image(tag: str):
    return docker_image_service.delete_image(tag=tag)

