FROM openjdk:17-bullseye
ARG jar_file
COPY $jar_file /application.jar
CMD ["java", "-jar", "/application.jar"]