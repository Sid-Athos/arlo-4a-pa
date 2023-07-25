FROM maven:3.9.3-eclipse-temurin-17
ARG GAME_FILE_NAME
COPY ./configs/pom.xml /pom.xml
COPY ./games/"${GAME_FILE_NAME}" /game.java
RUN mvn clean install
COPY ./target/game.jar /game.jar
