package com.groupawesome.apigraphdb.service;

import com.groupawesome.apigraphdb.model.Book;
import com.groupawesome.apigraphdb.repository.BookRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Collection;

@Service
public class BookService {

    @Autowired
    BookRepository bookRepository;

    public Collection<Book> getAll() { return bookRepository.getAllBooks(); }

    public Collection<Book> getTopTen(Integer num) {
        return bookRepository.getTopRatedBooks(num);
    }
}
