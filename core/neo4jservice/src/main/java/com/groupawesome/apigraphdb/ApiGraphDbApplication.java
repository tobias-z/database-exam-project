package com.groupawesome.apigraphdb;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@ComponentScan({
		"dk.groupa"
})

@SpringBootApplication
public class ApiGraphDbApplication {

	public static void main(String[] args) {
		SpringApplication.run(ApiGraphDbApplication.class, args);
	}

}
