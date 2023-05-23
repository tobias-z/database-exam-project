package com.groupawesome.apigraphdb.service;

import com.groupawesome.apigraphdb.model.User;
import com.groupawesome.apigraphdb.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Collection;

@Service
public class UserService {

    @Autowired
    UserRepository userRepository;

    public Collection<User> getAll() {
        return userRepository.getAllUsers();
    }
}
