package com.groupawesome.apigraphdb.model;


//import org.neo4j.ogm.annotation.Relationship;
import org.springframework.data.neo4j.core.schema.Relationship;
import org.springframework.data.neo4j.core.schema.GeneratedValue;
import org.springframework.data.neo4j.core.schema.Id;
import org.springframework.data.neo4j.core.schema.Node;

import javax.management.relation.Relation;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

@Node
public class Author {

    @Id
    private String name;

    @Relationship(type = "WRITTEN_BY", direction = Relationship.Direction.INCOMING)
//    private Set<Book> books = new HashSet<>();
    private List<Book> books;

    public Author() {

    }

    public Author(String name,
                  List<Book> books) {
//                  Set<Book> books) {
        this.name = name;
        this.books = books;
    }

    public String getName() {
        return name;
    }

//    public Set<Book> getBooks() {
//        return books;
//    }

    public List<Book> getBooks() {
        return books;
    }
}
