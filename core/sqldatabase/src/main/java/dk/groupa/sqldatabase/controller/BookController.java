package dk.groupa.sqldatabase.controller;

import dk.groupa.auth.Model.AuthDTO;
import dk.groupa.auth.Service.AuthenticationService;
import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.entity.WaitingBorrow;
import dk.groupa.sqldatabase.service.LoanService;
import dk.groupa.sqldatabase.service.ReserveService;
import jakarta.validation.constraints.NotNull;
import org.springframework.http.HttpHeaders;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/book")
public class BookController {
    private final LoanService loanService;
    private final ReserveService reserveService;
    private final AuthenticationService authenticationService;

    public BookController(LoanService loanService, ReserveService reserveService, AuthenticationService authenticationService) {
        this.loanService = loanService;
        this.reserveService = reserveService;
        this.authenticationService = authenticationService;
    }

    @PostMapping("/{bookId}/borrow")
    public ResponseEntity<Loan> borrowBook(@NotNull @PathVariable("bookId") Long bookId, @RequestHeader(HttpHeaders.AUTHORIZATION) String token) {
        AuthDTO authDTO = authenticationService.isUserAuthenticatedWithRole(token, "free", "subscribed");
        if (!authDTO.isAuthenticated()) {
            return ResponseEntity.status(401).build();
        }
        return ResponseEntity.ok(loanService.BorrowBook(authDTO.getUser().get().getId(), bookId));
    }

    @PostMapping("/{bookId}/return")
    public ResponseEntity<Loan> returnBook(@NotNull @PathVariable("bookId") Long bookId, @RequestHeader(HttpHeaders.AUTHORIZATION) String token) {
        AuthDTO authDTO = authenticationService.isUserAuthenticatedWithRole(token, "free", "subscribed");
        if (!authDTO.isAuthenticated()) {
            return ResponseEntity.status(401).build();
        }
        return ResponseEntity.ok(loanService.ReturnBook(authDTO.getUser().get().getId(), bookId));
    }

    @PostMapping("/{bookId}/reserve")
    public ResponseEntity<WaitingBorrow> reserveBook(@NotNull @PathVariable("bookId") Long bookId, @RequestHeader(HttpHeaders.AUTHORIZATION) String token) {
        AuthDTO authDTO = authenticationService.isUserAuthenticatedWithRole(token, "free", "subscribed");
        if (!authDTO.isAuthenticated()) {
            return ResponseEntity.status(401).build();
        }
        return ResponseEntity.ok(reserveService.Push(authDTO.getUser().get().getId(), bookId));
    }

    @PostMapping("/{bookId}/number")
    public ResponseEntity<Integer> getNumberInQueue(@NotNull @PathVariable("bookId") Long bookId, @RequestHeader(HttpHeaders.AUTHORIZATION) String token) {
        AuthDTO authDTO = authenticationService.isUserAuthenticatedWithRole(token, "free", "subscribed");
        if (!authDTO.isAuthenticated()) {
            return ResponseEntity.status(401).build();
        }
        return ResponseEntity.ok(reserveService.NumberInQue(authDTO.getUser().get().getId(), bookId));
    }
}