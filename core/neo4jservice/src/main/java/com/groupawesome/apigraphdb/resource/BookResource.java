package com.groupawesome.apigraphdb.resource;

import com.groupawesome.apigraphdb.model.Book;
import com.groupawesome.apigraphdb.service.BookService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.Collection;

@RestController
@RequestMapping("/api/neo4j/book")
public class BookResource {

    @Autowired
    BookService bookService;

    @GetMapping
    public Collection<Book> getAll() {
        return bookService.getAll();
    }

    @GetMapping("/topbooks/{limitNum}")
    public Collection<Book> getTopBooks(@PathVariable("limitNum") Integer num) { return bookService.getTopTen(num); }

}
