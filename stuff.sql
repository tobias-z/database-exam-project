-- Book_Title,Original_Book_Title,Author_Name,Edition_Language,Rating_score,Rating_votes,Review_number,Book_Description,Year_published,Genres,url

CREATE DATABASE youbook;

CREATE TABLE [dbo].[author] (
    [name] VARCHAR (255) NOT NULL,
    CONSTRAINT [PK_author] PRIMARY KEY CLUSTERED ([name] ASC)
);

CREATE TABLE [dbo].[genre] (
    [id]   BIGINT NOT NULL,
    [name] INT    NOT NULL,
    CONSTRAINT [PK_genre] PRIMARY KEY CLUSTERED ([id] ASC)
);

CREATE TABLE [dbo].[book] (
    [id]             BIGINT        NOT NULL,
    [title]          VARCHAR (100) NOT NULL,
    [description]    TEXT          NULL,
    [author_name]    VARCHAR (255) NOT NULL,
    [language]       VARCHAR (100) NOT NULL,
    [rating_score]   DECIMAL (2)   NULL,
    [rating_votes]   BIGINT        CONSTRAINT [DEFAULT_book_rating_votes] DEFAULT 0 NOT NULL,
    [review_number]  BIGINT        CONSTRAINT [DEFAULT_book_review_number] DEFAULT 0 NOT NULL,
    [year_published] INT           NOT NULL,
    CONSTRAINT [PK_book] PRIMARY KEY CLUSTERED ([id] ASC),
    CONSTRAINT [FK_book_author] FOREIGN KEY ([author_name]) REFERENCES [dbo].[author] ([name])
);

CREATE TABLE [dbo].[book_genre] (
    [book_id]   BIGINT NOT NULL,
    [genre_id]  BIGINT NOT NULL,
    [voted_for] INT    NULL,
    CONSTRAINT [PK_book_genre] PRIMARY KEY CLUSTERED ([book_id] ASC, [genre_id] ASC),
    CONSTRAINT [FK_book_genre_book] FOREIGN KEY ([book_id]) REFERENCES [dbo].[book] ([id]),
    CONSTRAINT [FK_book_genre_genre] FOREIGN KEY ([genre_id]) REFERENCES [dbo].[genre] ([id])
);

CREATE TABLE [dbo].[user] (
    [id]       BIGINT        NOT NULL,
    [email]    VARCHAR (50)  NULL,
    [password] VARCHAR (255) NULL,
    [role]     VARCHAR (20)  NULL,
    CONSTRAINT [PK_user] PRIMARY KEY CLUSTERED ([id] ASC),
    CONSTRAINT [role_constraint] CHECK ([role]='empolyee' OR [role]='free' OR [role]='subscribed')
);

GO
EXECUTE sp_addextendedproperty @name = N'MS_Description', @value = N'A user should only be subscribed free or an employee', @level0type = N'SCHEMA', @level0name = N'dbo', @level1type = N'TABLE', @level1name = N'user', @level2type = N'CONSTRAINT', @level2name = N'role_constraint';

-- Books available
-- The user with the highest priority is granted a book if there are any books available

-- 100 - 105

-- Counter for every book which counts the current index in the borrowing queue
-- When a person is pushed to the queue, he is
