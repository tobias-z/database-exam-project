package dk.groupa.sqldatabase.controller;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.service.LoanService;
import jakarta.validation.constraints.NotNull;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/book")
public class BookController {

    private final LoanService loanService;

    public BookController(LoanService loanService) {
        this.loanService = loanService;
    }

    @PostMapping("/{bookId}/borrow")
    public Loan borrowBook(@NotNull @PathVariable("bookId") Integer bookId) {
        int userId = 1;     //TODO: Replace med Mads auth service
        return loanService.BorrowBook(bookId, userId);
    }

    @PostMapping("/{bookId}/return")
    public Loan returnBook(@NotNull @PathVariable("bookId") Integer bookId) {
        int userId = 1;     //TODO: Replace med Mads auth service
        return loanService.ReturnBook(bookId, userId);
    }
}