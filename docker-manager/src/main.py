from fastapi import FastAPI
from .routers import docker_images, docker_container

app = FastAPI()

app.include_router(docker_images.router)
app.include_router(docker_container.router)


@app.head("/")
def head():
    return
