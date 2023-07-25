FROM maven:3.9.3-eclipse-temurin-17
ARG GAME_FILE_NAME
COPY ./configs/pom.xml /pom.xml
COPY ./games/"${GAME_FILE_NAME}" /main.java
RUN mvn clean install
RUN cp /target/game.jar /game.jar
