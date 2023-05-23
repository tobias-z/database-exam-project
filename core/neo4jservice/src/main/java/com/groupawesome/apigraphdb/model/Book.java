package com.groupawesome.apigraphdb.model;

import org.springframework.data.neo4j.core.schema.Relationship;
import org.springframework.data.neo4j.core.schema.GeneratedValue;
import org.springframework.data.neo4j.core.schema.Id;
import org.springframework.data.neo4j.core.schema.Node;

@Node
public class Book {

    @Id
    private Long id;
    private String title;
    private String author;
    private Double avgRating;
    private Integer yearPublished;

    public Book() {
    }

    public Book(Long id, String title, String author, Double avgRating, Integer yearPublished) {
        this.id = id;
        this.title = title;
        this.author = author;
        this.avgRating = avgRating;
        this.yearPublished = yearPublished;
    }

    public Long getId() {
        return id;
    }

    public String getTitle() {
        return title;
    }

    public String getAuthor() {
        return author;
    }

    public Double getAvgRating() {
        return avgRating;
    }

    public Integer getYearPublished() {
        return yearPublished;
    }
}
