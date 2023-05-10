package dk.groupa.auth.Utils;

import org.springframework.stereotype.Component;

import java.util.Base64;

@Component
public class AuthTokenDecode {
    public String[] decodeBase64Token(String token) {
        return new String(Base64.getDecoder().decode(token.split("Basic ")[1])).split(":");
    }
}