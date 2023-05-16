package dk.groupa.authservice.domain.service;

import java.util.List;
import java.util.Set;
import lombok.RequiredArgsConstructor;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class UserCachingService {

    private static final Set<String> EMAILS = Set.of("cph-tz11@cphbusiness.dk");

    private final RedisTemplate<String, String> redisTemplate;

    public Set<String> getEmailsOfRole(String role) {
        return EMAILS;
//        return redisTemplate.opsForSet().members(role);
    }

    public void saveEmailsInRole(String role, List<String> emails) {
        this.redisTemplate.opsForSet().add(role, emails.toArray(new String[0]));
    }

}
