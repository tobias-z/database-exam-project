package dk.groupa.auth.Utils;

import org.springframework.stereotype.Component;

import java.util.Base64;
import java.util.Optional;

@Component
public class AuthTokenDecode {
    public Optional<String[]> decodeBase64Token(String token) {
        if(token.contains("Basic ")) {
            String[] base64encoded = token.split("Basic ");

            if(base64encoded[1].length() > 0) {
                String base64decoded = new String(Base64.getDecoder().decode(base64encoded[1]));

                if(base64decoded.contains(":")) {
                    String[] usernameAndPassword = base64decoded.split(":");
                    return Optional.of(usernameAndPassword);
                }
            }
        }
        return Optional.empty();
    }
}