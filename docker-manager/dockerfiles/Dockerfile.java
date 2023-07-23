FROM amazoncorretto:latest
ARG GAME_FILE_NAME
COPY ./games/"${GAME_FILE_NAME}" /game.java
RUN javac game.java