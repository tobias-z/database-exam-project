package dk.groupa.sqldatabase.controller;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.entity.WaitingBorrow;
import dk.groupa.sqldatabase.service.LoanService;
import dk.groupa.sqldatabase.service.ReserveService;
import jakarta.validation.constraints.NotNull;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/book")
public class BookController {
    private final LoanService loanService;
    private final ReserveService reserveService;

    public BookController(LoanService loanService, ReserveService reserveService) {
        this.loanService = loanService;
        this.reserveService = reserveService;
    }

    @PostMapping("/{bookId}/borrow")
    public Loan borrowBook(@NotNull @PathVariable("bookId") Long bookId) {
        Long userId = Long.valueOf(1);     //TODO: Replace med Mads auth service
        return loanService.BorrowBook(bookId, userId);
    }

    @PostMapping("/{bookId}/return")
    public Loan returnBook(@NotNull @PathVariable("bookId") Long bookId) {
        Long userId = Long.valueOf(1);     //TODO: Replace med Mads auth service
        return loanService.ReturnBook(bookId, userId);
    }

    @PostMapping("/{bookId}/reserve")
    public WaitingBorrow reserveBook(@NotNull @PathVariable("bookId") Long bookId) {
        Long userId = Long.valueOf(1);     //TODO: Replace med Mads auth service
        return reserveService.Push(userId, bookId);
    }
}