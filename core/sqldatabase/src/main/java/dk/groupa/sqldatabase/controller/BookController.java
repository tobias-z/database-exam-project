package dk.groupa.sqldatabase.controller;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.service.BorrowService;
import jakarta.websocket.server.PathParam;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/book")
public class BookController {

    private final BorrowService borrowService;

    public BookController(BorrowService borrowService) {
        this.borrowService = borrowService;
    }

    @PostMapping("/{bookId}/borrow")
    public Loan borrowBook(@PathVariable("bookId") Integer bookId){
        int userId = 1;     //TODO: Replace med Mads auth service
        return borrowService.BorrowBook(bookId, userId);
    }
}