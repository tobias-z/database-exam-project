package dk.groupa.datatransfomer.Model.MSSQL;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.util.Objects;

@NoArgsConstructor
@AllArgsConstructor
@Getter
@Setter
public class AuthorMS {
    private String name;

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        AuthorMS authorMS = (AuthorMS) o;
        return Objects.equals(name, authorMS.name);
    }

    @Override
    public int hashCode() {
        return Objects.hash(name);
    }
}
