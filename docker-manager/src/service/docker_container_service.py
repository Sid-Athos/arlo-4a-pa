from env import DOCKER_USERNAME, DOCKERHUB_REPOSITORY
from ..dependencies.docker import docker_client
from .docker_image_service import pull_image


def run_container(tag: str) -> str:
    if pull_image(tag=tag):
        docker_client.containers.run(f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}:{tag}", name="test", detach=True,
                                     auto_remove=True)

    return ""
