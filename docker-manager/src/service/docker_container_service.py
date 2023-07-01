import json

import docker
from docker.models.containers import Container, ExecResult

from env import DOCKER_USERNAME, DOCKERHUB_REPOSITORY
from ..dependencies.docker import docker_client
from .docker_image_service import pull_image


def run_container(tag: str) -> str:
    if pull_image(tag=tag):
        run_output: Container = docker_client.containers.run(image=f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}:{tag}",
                                                             detach=True,
                                                             auto_remove=False,
                                                             tty=True)
        if run_output:
            return run_output.id
    return ""


def exec_on_container(container_id: str, command: str) -> dict | bool:
    try:
        container: Container = docker_client.containers.get(container_id=container_id)
    except (docker.errors.NotFound, docker.errors.APIError):
        return False
    result: ExecResult = container.exec_run(cmd=json.dumps(command), stdout=True)
    json_obj: dict = json.loads(result.output)
    return json_obj


def close_container(container_id: str) -> bool:
    try:
        container: Container = docker_client.containers.get(container_id=container_id)
    except (docker.errors.NotFound, docker.errors.APIError):
        return False
    container.stop()
    return True
