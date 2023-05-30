package com.groupawesome.apigraphdb.resource;

import com.groupawesome.apigraphdb.model.Author;
import com.groupawesome.apigraphdb.service.AuthorService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Collection;

@RestController
@RequestMapping("/author")
public class AuthorResource {

    @Autowired
    AuthorService authorService;

    @GetMapping
    public Collection<Author> getAll() {
        return authorService.getAll();
    }

    @GetMapping("/{name}")
    public Collection<Author> getByName(@PathVariable("name") String name) { return authorService.getByName(name); }
}
