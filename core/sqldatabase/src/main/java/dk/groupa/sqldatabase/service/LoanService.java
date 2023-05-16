package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.repository.LoanRepository;
import org.springframework.stereotype.Service;

@Service
public class LoanService {
    private final LoanRepository loanRepository;
    private final BookService bookService;

    private final ReserveService reserveService;



    public LoanService(LoanRepository loanRepository, BookService bookService, ReserveService reserveService) {
        this.loanRepository = loanRepository;
        this.bookService = bookService;
        this.reserveService = reserveService;
    }

    public Loan BorrowBook(int userId, int bookId) {

        return loanRepository.borrowBook(userId, bookId);

    }

    public Loan ReturnBook(int userId, int bookId) {  //TODO: Hvis en user har lånt den samme bog 2 gange bliver begge returned men kun 1 ligges på lager
        Loan loan = loanRepository.returnBook(userId, bookId);
        reserveService.Pop(bookId);
        return loan;
    }
}