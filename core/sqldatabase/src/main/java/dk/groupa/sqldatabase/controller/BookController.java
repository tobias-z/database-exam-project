package dk.groupa.sqldatabase.controller;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.service.BorrowService;
import dk.groupa.sqldatabase.service.ReturnService;
import jakarta.validation.constraints.NotNull;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/book")
public class BookController {

    private final BorrowService borrowService;
    private final ReturnService returnService;

    public BookController(BorrowService borrowService, ReturnService returnService) {
        this.borrowService = borrowService;
        this.returnService = returnService;
    }

    @PostMapping("/{bookId}/borrow")
    public Loan borrowBook(@NotNull @PathVariable("bookId") Integer bookId) {
        int userId = 1;     //TODO: Replace med Mads auth service
        return borrowService.BorrowBook(bookId, userId);
    }

    @PostMapping("/{bookId}/return")
    public Loan returnBook(@NotNull @PathVariable("bookId") Integer bookId) {
        int userId = 1;     //TODO: Replace med Mads auth service
        return returnService.ReturnBook(bookId, userId);
    }
}