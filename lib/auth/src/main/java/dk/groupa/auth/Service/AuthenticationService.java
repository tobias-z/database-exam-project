package dk.groupa.auth.Service;

import dk.groupa.auth.Model.AuthDTO;
import dk.groupa.auth.Model.User;
import dk.groupa.auth.Repo.AuthenticationRepo;
import dk.groupa.auth.Utils.AuthTokenDecode;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Objects;
import java.util.Optional;

@Service
public class AuthenticationService {

    @Autowired
    private AuthTokenDecode authTokenDecode;

    @Autowired
    AuthenticationRepo authenticationRepo;

    public AuthDTO isUserAuthenticatedWithRole(String authToken, String role) {
        Optional<String[]> decoded = authTokenDecode.decodeBase64Token(authToken);

        if(decoded.isEmpty()) {
            return new AuthDTO(Optional.of(new User()), false);
        }

        Optional<User> user = authenticationRepo.findUserByEmailAndPassword(decoded.get()[0], decoded.get()[1]);

        if(user.isPresent()) {
            if(Objects.equals(user.get().getRole(), role)) {
                return new AuthDTO(user, true);
            }
        }
        return new AuthDTO(user, false);
    }

    public User createUser(String email, String password, String role) {
        return authenticationRepo.createUser(email, password, role);
    }
}
