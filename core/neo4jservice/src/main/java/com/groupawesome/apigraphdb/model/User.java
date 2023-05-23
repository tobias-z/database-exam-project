package com.groupawesome.apigraphdb.model;

import org.neo4j.ogm.annotation.Relationship;
import org.springframework.data.neo4j.core.schema.Id;
import org.springframework.data.neo4j.core.schema.Node;

import java.util.List;

@Node
public class User {

    @Id
    private Long id;
    private String email;
    private String role;

    @Relationship(type = "BORROWED")
    private List<Book> bookList;

    public User() {

    }

    public Long getId() {
        return id;
    }

    public String getEmail() {
        return email;
    }

    public String getRole() {
        return role;
    }

    public List<Book> getBookList() {
        return bookList;
    }
}
