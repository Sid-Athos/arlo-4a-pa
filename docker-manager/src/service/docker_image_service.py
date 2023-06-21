import os

from ..dependencies.docker import dockerClient


def build_image(language: str, tag: str) -> str:
    build_output = dockerClient.images.build(path=f"{os.getcwd()}/dockerfiles/{language}", tag=[f"dixennaxos/arlo-4a-pa-games:{tag}"])
    #if build_output[1].contains("Success"):
    return f"dixennaxos/arlo-4a-pa-games:{tag}"
    return ""


def push_image(tag: str) -> str:
    build_output = dockerClient.images.push(tag)
    print(str(build_output))
    for line in build_output[1]:
        print(line)
    return "OK"
