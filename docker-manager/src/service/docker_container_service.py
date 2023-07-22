import time

import docker
from docker.models.containers import Container

from env import DOCKER_USERNAME, DOCKERHUB_REPOSITORY
from ..dependencies.docker import docker_client
from .docker_image_service import pull_image

import subprocess

languages_commands = {"python3": ["python3", "main.py"], "c": ["./game"], "rust": ["./game"],
                      "java": ["java", "game"]}


def run_container(tag: str) -> str:
    if pull_image(tag=tag):
        run_output: Container = docker_client.containers.run(image=f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}:{tag}",
                                                             detach=True,
                                                             tty=True,
                                                             auto_remove=False,
                                                             stdin_open=True,
                                                             stdout=True)
        if run_output:
            return run_output.id
    return ""


def exec_on_container(tag: str, language: str, commands: list[str]) -> str or bool:
    pull_image(tag=tag)
    process = subprocess.Popen(
        ["docker", "run", "-i", "--rm", f"{DOCKER_USERNAME}/{DOCKERHUB_REPOSITORY}:{tag}",
         *languages_commands[f'{language}']],
        stdout=subprocess.PIPE, stdin=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)

    res = ""
    for command in commands:
        process.stdin.write(f'{command}\n')
        process.stdin.flush()
        time.sleep(1)
        brackets = {"opening": 0, "closing": 0}
        res = ""
        while brackets["opening"] < 1 or brackets["opening"] != brackets["closing"]:
            o = process.stdout.readline()
            if '{' in o:
                brackets["opening"] += 1
            if '}' in o:
                brackets["closing"] += 1
            res += o
            if brackets["opening"] == brackets["closing"]:
                if brackets["opening"] == 0:
                    return False
                break
    process.kill()
    res = res.replace(" ", "")
    return res.replace("\n", "")


def close_container(container_id: str) -> bool:
    try:
        container: Container = docker_client.containers.get(container_id=container_id)
    except (docker.errors.NotFound, docker.errors.APIError):
        return False
    container.stop()
    docker_client.containers.prune()
    return True
