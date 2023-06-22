from fastapi import APIRouter

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
    docker_container_service.run_container(image_name)
