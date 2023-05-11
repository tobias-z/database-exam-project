package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.repository.ReturnRepository;
import org.springframework.stereotype.Service;

@Service
public class ReturnService {

    private final ReturnRepository returnRepository;

    public ReturnService(ReturnRepository returnRepository) {
        this.returnRepository = returnRepository;
    }

    public Loan ReturnBook(int bookId, int userId) {
        return returnRepository.returnBook(userId, bookId);
    }
}
