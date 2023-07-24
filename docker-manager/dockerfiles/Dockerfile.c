FROM gcc:4.9
ARG GAME_FILE_NAME
COPY ./games/"${GAME_FILE_NAME}" /main.c
RUN gcc -o game main.c