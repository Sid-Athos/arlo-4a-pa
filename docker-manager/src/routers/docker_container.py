from fastapi import APIRouter

router = APIRouter(
    prefix="/containers",
    tags=["Containers"],
    dependencies=[],
    responses={404: {"description": "Not found"}},
)


@router.head("/")
async def head():
    return
