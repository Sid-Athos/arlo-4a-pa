from docker import DockerClient
from env import DOCKER_USERNAME, DOCKER_PASSWORD
docker_client = DockerClient.from_env()
docker_client.login(username=DOCKER_USERNAME, password=DOCKER_PASSWORD)
