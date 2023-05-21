package dk.groupa.sqldatabase;

import org.springframework.context.annotation.ComponentScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@ComponentScan({
		"dk.groupa"
})

@SpringBootApplication
public class SqldatabaseApplication {

	public static void main(String[] args) {
		SpringApplication.run(SqldatabaseApplication.class, args);
	}
}