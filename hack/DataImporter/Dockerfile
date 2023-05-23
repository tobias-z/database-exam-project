FROM openjdk:17-bullseye
ARG jar_file
COPY $jar_file /application.jar

# Create the folder where the application will look for the .csv files.
RUN mkdir /csv/

# unzip the the file and select the /csv folder as the destination.
RUN unzip /src/main/resources/static/books/Science_Fiction_Books.zip -d /csv

CMD ["java", "-jar", "/application.jar"]