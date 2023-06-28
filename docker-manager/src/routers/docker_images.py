from fastapi import APIRouter, status
from fastapi.responses import JSONResponse, Response

from ..model import ImageBuildModel
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


@router.post("/", status_code=200)
async def build_image(image_build_model: ImageBuildModel):
    print(image_build_model)
    tag = docker_image_service.build_image(language=image_build_model.language, tag=image_build_model.tag,
                                           game_file_name=image_build_model.game_file_name)
    if tag != "":
        if docker_image_service.push_image(tag=tag) == "OK":
            return JSONResponse(status_code=status.HTTP_201_CREATED, content={"tag": tag})
    return Response(status_code=status.HTTP_400_BAD_REQUEST)


@router.delete("/", status_code=204)
async def delete_image(tag: str):
    return docker_image_service.delete_image(tag=tag)
