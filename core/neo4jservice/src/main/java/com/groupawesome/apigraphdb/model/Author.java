package com.groupawesome.apigraphdb.model;


import org.neo4j.ogm.annotation.GraphId;
import org.neo4j.ogm.annotation.NodeEntity;
import org.neo4j.ogm.annotation.Relationship;

import java.util.List;

@NodeEntity
public class Author {

    @GraphId
    private Long id;
    private String name;
    private Integer age;

    @Relationship(type = "WROTE")
    private List<Book> bookList;

    public Author() {

    }

    public Long getId() {
        return id;
    }

    public String getName() {
        return name;
    }

    public Integer getAge() {
        return age;
    }
}
