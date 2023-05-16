package dk.groupa.authservice.domain.service;

import java.util.List;
import org.springframework.stereotype.Service;

@Service
public class UserCachingService {

    private List<String> EMAILS = List.of("cph-tz11@cphbusiness.dk");

    public List<String> getEmailsOfRole(String role) {
        return EMAILS;
    }

}
