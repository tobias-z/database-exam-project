package com.groupawesome.apigraphdb.service;

import com.groupawesome.apigraphdb.model.Author;
import com.groupawesome.apigraphdb.repository.AuthorRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Collection;

@Service
public class AuthorService {

    @Autowired
    AuthorRepository authorRepository;

    public Collection<Author> getAll() { return authorRepository.getAllAuthors(); }

    public Collection<Author> getByName(String name) { return authorRepository.getAuthorByName(name); }
}
