package dk.groupa.authservice.domain.service;

import java.time.Duration;
import java.util.List;
import java.util.Set;
import lombok.RequiredArgsConstructor;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class UserCachingService {

    // not sure what is the best timeout here tbh
    private static final Duration ROLE_TIMEOUT = Duration.ofDays(7);

    private final RedisTemplate<String, String> redisTemplate;

    public Set<String> getEmailsOfRole(String role) {
        return this.redisTemplate.opsForSet().members(role);
    }

    public void saveEmailsInRole(String role, List<String> emails) {
        this.redisTemplate.opsForSet().add(role, emails.toArray(new String[0]));
        this.redisTemplate.expire(role, ROLE_TIMEOUT);
    }

}
