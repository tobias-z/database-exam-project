package dk.groupa.auth.Model;

import java.util.Optional;

public class AuthDTO {
    Optional<User> user;
    boolean isAuthenticated;

    public AuthDTO(Optional<User> user, boolean isAuthenticated) {
        this.user = user;
        this.isAuthenticated = isAuthenticated;
    }

    public Optional<User> getUser() {
        return user;
    }

    public void setUser(Optional<User> user) {
        this.user = user;
    }

    public boolean isAuthenticated() {
        return isAuthenticated;
    }

    public void setAuthenticated(boolean authenticated) {
        isAuthenticated = authenticated;
    }
}
