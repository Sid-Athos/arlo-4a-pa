from ..dependencies.docker import docker_client


def create_container(image_name) -> str:
    docker_client.containers.create(f"{image_name}")
    return ""
