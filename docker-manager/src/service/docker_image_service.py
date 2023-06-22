import json
import os
import requests
from docker.models.images import Image
from requests import Response

from ..dependencies.docker import docker_client
from env import DOCKERHUB_REPOSITORY, DOCKER_USERNAME, DOCKER_PASSWORD


def build_image(language: str, tag: str) -> str:
    build_output: list = docker_client.images.build(path=f"{os.getcwd()}/dockerfiles/{language}",
                                                    tag=[f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}:{tag}"],
                                                    buildargs={
                                                        "FILE_PATH": f"{os.getcwd()}/dockerfiles/{language}/Dockerfile"})
    for line in build_output[1]:
        if "stream" in line and "Successfully tagged" in line["stream"]:
            return f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}:{tag}"
    return ""


def push_image(tag: str) -> str:
    push_output = docker_client.images.push(tag)
    if "digest" in push_output:
        return "OK"
    return "Not OK"


def pull_image(tag: str) -> bool:
    pull_output: Image = docker_client.images.pull(repository=f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}", tag=tag)
    if pull_output:
        return True
    return False


def delete_image_from_repository(tag: str) -> bool:
    response: Response = requests.post(
        'https://hub.docker.com/v2/users/login',
        json={'username': DOCKER_USERNAME, 'password': DOCKER_PASSWORD}
    )
    access_token: str = response.json()['token']

    headers: dict = {'Authorization': f'Bearer {access_token}'}
    url: str = f'https://hub.docker.com/v2/repositories/{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}/tags/{tag}/'
    response: Response = requests.delete(url, headers=headers)

    if response.status_code == 204:
        return True
    else:
        return False


def delete_image(tag: str) -> bool:
    docker_client.images.remove(image=f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}:{tag}")
    return delete_image_from_repository(tag=tag)
