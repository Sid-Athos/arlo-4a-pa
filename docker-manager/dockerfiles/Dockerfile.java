FROM maven:3.9.3-eclipse-temurin-17
COPY ./configs/pom.xml /pom.xml
COPY ./games/morpion.java /game.java
RUN mvn clean install
COPY ./target/game.jar /game.jar
