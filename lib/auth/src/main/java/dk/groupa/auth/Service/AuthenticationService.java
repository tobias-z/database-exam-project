package dk.groupa.auth.Service;

import dk.groupa.auth.Model.AuthDTO;
import dk.groupa.auth.Model.User;
import dk.groupa.auth.Repo.AuthenticationRepo;
import dk.groupa.auth.Utils.AuthTokenDecode;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Objects;
import java.util.Optional;

@Service
public class AuthenticationService {

    @Autowired
    private AuthTokenDecode authTokenDecode;

    @Autowired
    AuthenticationRepo authenticationRepo;

    public AuthDTO isUserAuthenticatedWithRole(String authToken, String role) {
        String[] decoded = authTokenDecode.decodeBase64Token(authToken);

        Optional<User> user = authenticationRepo.findUserByEmailAndPassword(decoded[0], decoded[1]);

        if(user.isPresent()) {
            if(Objects.equals(user.get().getRole(), role)) {
                return new AuthDTO(user, true);
            }
        }
        return new AuthDTO(user, false);
    }
}
