package dk.groupa.auth.Repo;

import dk.groupa.auth.Model.User;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.Optional;

@Repository
public interface AuthenticationRepo extends JpaRepository<User, Long> {
    @Query(value = "SELECT * FROM dbo.[user] WHERE email = :email AND password = :password", nativeQuery = true)
    Optional<User> findUserByEmailAndPassword(@Param("email") String email, @Param("password") String password);

    @Query(value = "EXEC sp_CreateUser @email = :email, @password = :password, @role = :role", nativeQuery = true)
    User createUser(@Param("email") String email, @Param("password") String password, @Param("role") String role);

    @Query(value = "SELECT email FROM dbo.[user] WHERE role = :email")
    List<String> getEmailsByRole(@Param("role") String role);
}
