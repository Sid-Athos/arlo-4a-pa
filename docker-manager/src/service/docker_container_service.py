from env import DOCKER_USERNAME, DOCKERHUB_REPOSITORY
from ..dependencies.docker import docker_client
from .docker_image_service import pull_image


def create_container(tag: str) -> str:
    pull_image(tag=tag)
    docker_client.containers.create(f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}:{tag}")
    return ""
