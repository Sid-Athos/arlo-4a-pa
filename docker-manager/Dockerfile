FROM debian:bullseye-slim
RUN apt update && apt upgrade -y
RUN apt install python3 python3-pip python3-venv apt-transport-https ca-certificates curl gnupg lsb-release -y
RUN install -m 0755 -d /etc/apt/keyrings
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
RUN chmod a+r /etc/apt/keyrings/docker.gpg
RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" > /etc/apt/sources.list.d/docker.list
RUN apt update
RUN apt -y install docker-ce docker-ce-cli containerd.io


WORKDIR /code
RUN python3 -m venv /code/venv
COPY ./ /code
RUN pip install --no-cache-dir --upgrade -r /code/requirements.txt
EXPOSE 7588
CMD ["uvicorn", "src.main:app", "--host", "0.0.0.0", "--port", "7588"]