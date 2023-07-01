from fastapi import APIRouter, status
from fastapi.responses import JSONResponse, Response

from ..service import docker_container_service

router = APIRouter(
    prefix="/containers",
    tags=["Containers"],
    dependencies=[],
    responses={404: {"description": "Not found"}},
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


@router.post("/{image_name}")
async def exec_on_container(image_name: str, commands: list[str] | str):
    container_id = docker_container_service.run_container(image_name)
    if container_id == "":
        return Response(status_code=status.HTTP_400_BAD_REQUEST)

    if commands is list:
        for command in commands:
            content: dict | bool = docker_container_service.exec_on_container(container_id=container_id,
                                                                              command=command)
    else:
        content: dict | bool = docker_container_service.exec_on_container(container_id=container_id, command=commands)

    docker_container_service.close_container(container_id=container_id)

    if content:
        return JSONResponse(status_code=status.HTTP_200_OK, content=content)
    return Response(status_code=status.HTTP_400_BAD_REQUEST)
