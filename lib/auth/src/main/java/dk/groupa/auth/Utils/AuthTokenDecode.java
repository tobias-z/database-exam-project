package dk.groupa.auth.Utils;

import org.springframework.stereotype.Component;

import java.util.Base64;
import java.util.Optional;

@Component
public class AuthTokenDecode {
    public Optional<String[]> decodeBase64Token(String token) {
        if(token.contains("Basic ")) {
            String[] base64encoded = token.split("Basic ");

            // 4 seems to be the lowest possible encoded base64 length for a username and password in the Authorization header.
            if(base64encoded[1].length() >= 4) {
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