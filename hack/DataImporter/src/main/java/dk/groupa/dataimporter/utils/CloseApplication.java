package dk.groupa.dataimporter.utils;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.SpringApplication;
import org.springframework.context.ApplicationContext;
import org.springframework.stereotype.Component;

@Component
public class CloseApplication {
    @Autowired
    private ApplicationContext appContext;

    public void exit(Integer exitCode) {
        System.exit(SpringApplication.exit(appContext, () -> exitCode));
    }

    public void error(String functionName, String reason) {
        System.out.printf("An error occurred during runtime. This happened in the function: %s - which might be the result of %s", functionName, reason);
        System.exit(SpringApplication.exit(appContext, () -> 1));
    }
}
