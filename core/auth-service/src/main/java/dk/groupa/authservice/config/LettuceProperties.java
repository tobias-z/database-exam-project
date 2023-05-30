package dk.groupa.authservice.config;

import lombok.Getter;
import lombok.Setter;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Configuration;

@Configuration
@ConfigurationProperties("dk.groupa.redis")
@Getter
@Setter
public class LettuceProperties {

    private String host = "localhost";
    private Integer port = 6379;
}
