from fastapi import APIRouter, status
from fastapi.responses import JSONResponse, Response

from ..model import ContainerExecModel
from ..service import docker_container_service

router = APIRouter(
    prefix="/containers",
    tags=["Containers"],
    dependencies=[],
    responses={404: {"description": "Not found"}, 400: {"description": "Something went wrong"},
               201: {"description": "Created"}, 200: {"description": "Success"}},
)


@router.head("/")
async def head():
    return


@router.post("/")
async def run_container(image_name: str):
    container_id = docker_container_service.run_container(image_name)
    if container_id != "":
        return JSONResponse(status_code=status.HTTP_201_CREATED, content={"container_id": container_id})
    return Response(status_code=status.HTTP_400_BAD_REQUEST)


@router.post("/{tag}")
async def exec_on_container(tag: str, container_exec_model: ContainerExecModel):
    content: str = docker_container_service.exec_on_container(tag=tag,
                                                              language=container_exec_model.language,
                                                              commands=container_exec_model.commands)

    if content:
        return JSONResponse(status_code=status.HTTP_200_OK, content=content)
    return Response(status_code=status.HTTP_400_BAD_REQUEST)
