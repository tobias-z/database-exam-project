--Create procedure for users to borrow books
CREATE OR ALTER PROCEDURE [dbo].[sp_BorrowBook]

    @user_id BIGINT,
    @book_id BIGINT

    AS
        BEGIN TRY
        BEGIN TRANSACTION borrowBook
                BEGIN
                    INSERT INTO loans (user_id, book_id, borrowed_at, due_date) values (@user_id, @book_id, (Select Getdate()), (Select Getdate()+30));
                    UPDATE book SET available = 1 WHERE id = @book_id;
                END
        COMMIT TRANSACTION borrowBook
        END TRY

        BEGIN CATCH
            ROLLBACK TRANSACTION borrowBook;
            BEGIN
                    RAISERROR ('failed to insert loan', 1, 1);
            END
        END CATCH
GO

--Create procedure for employees to create new authors
CREATE OR ALTER PROCEDURE [dbo].[sp_CreateAuthor]

    @name VARCHAR(255)

    AS
        BEGIN TRY
        BEGIN TRANSACTION createAuthor
                BEGIN
                    INSERT INTO author (name) values (@name);
                END
        COMMIT TRANSACTION createAuthor
        END TRY

        BEGIN CATCH
            ROLLBACK TRANSACTION createAuthor;
            BEGIN
                    RAISERROR ('failed to insert author', 1, 1);
            END
        END CATCH
GO

--Create procedure for employees to create new books
CREATE OR ALTER PROCEDURE [dbo].[sp_CreateBook]

    @title VARCHAR(100),
    @description text,
    @author_name VARCHAR(255),
    @language VARCHAR(100),
    @year_published INT

    AS
        BEGIN TRY
        BEGIN TRANSACTION createBook
                BEGIN
                    INSERT INTO book (title, [description], author_name, [language], year_published) values (@title, @description, @author_name, @language, @year_published);
                END
        COMMIT TRANSACTION createBook
        END TRY

        BEGIN CATCH
            ROLLBACK TRANSACTION createBook;
            BEGIN
                    RAISERROR ('failed to insert book', 1, 1);
            END
        END CATCH
GO

--Create procedure for employees to create new users
CREATE OR ALTER PROCEDURE [dbo].[sp_CreateUser]

    @email VARCHAR(50),
    @password VARCHAR(255),
    @role VARCHAR(20)

    AS
        BEGIN TRY
        BEGIN TRANSACTION createUser
                BEGIN
                    INSERT INTO [user] (email, [password], role) VALUES (@email, @password, @role);
                    SELECT * FROM [user] WHERE id = SCOPE_IDENTITY();
                END
        COMMIT TRANSACTION createUser
        END TRY

        BEGIN CATCH
            ROLLBACK TRANSACTION createUser;
            BEGIN
                    RAISERROR ('failed to insert user', 1, 1);
            END
        END CATCH
GO

--Create procedure for users to return books
CREATE OR ALTER PROCEDURE [dbo].[sp_ReturnBook]

    @user_id BIGINT,
    @book_id BIGINT

    AS
        BEGIN TRY
        BEGIN TRANSACTION returnBook
                BEGIN
                    UPDATE loans SET returned_at = (Select GETDATE()) WHERE user_id = @user_id and book_id = @book_id;
                    UPDATE book SET available = 0 WHERE id = @book_id;
                END
        COMMIT TRANSACTION returnBook
        END TRY

        BEGIN CATCH
            ROLLBACK TRANSACTION returnBook;
            BEGIN
                    RAISERROR ('failed to return loan', 1, 1);
            END
        END CATCH
GO

--Create procedure for users to reserve a book
CREATE OR ALTER PROCEDURE [dbo].[sp_ReserveBook]

    @user_id BIGINT,
    @book_id BIGINT

    AS
        BEGIN TRY
        BEGIN TRANSACTION reserveBook
            BEGIN
                INSERT INTO borrow_queue (user_id, book_id, enqueued_at) values (@user_id, @book_id, (Select Getdate()));
            END

        COMMIT TRANSACTION reserveBook
        END TRY

        BEGIN CATCH
            ROLLBACK TRANSACTION reserveBook;
            BEGIN
                RAISERROR ('failed to insert reservation', 1, 1);
            END
        END CATCH
GO