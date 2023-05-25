package dk.groupa.datatransfomer;

import dk.groupa.datatransfomer.Service.UpdateService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.scheduling.annotation.EnableScheduling;
import org.springframework.scheduling.annotation.Scheduled;

@SpringBootApplication
@EnableScheduling
public class DataTransformerApplication implements CommandLineRunner {
    @Autowired
    UpdateService updateService;

    public static void main(String[] args) {
        SpringApplication.run(DataTransformerApplication.class, args);
    }

    @Override
    public void run(String... args) {
        //updateService.updateNeo4jData();
    }

}
