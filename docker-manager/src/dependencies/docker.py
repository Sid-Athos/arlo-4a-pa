from docker import DockerClient
from env import DOCKER_USERNAME, DOCKER_PASSWORD
dockerClient = DockerClient.from_env()
dockerClient.login(username=DOCKER_USERNAME, password=DOCKER_PASSWORD)
