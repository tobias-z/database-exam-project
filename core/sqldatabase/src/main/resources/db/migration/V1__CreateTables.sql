--Create table for authors
CREATE TABLE [dbo].[author](
	[name] [varchar](255) NOT NULL
) ON [PRIMARY]
GO
SET ANSI_PADDING ON
GO
ALTER TABLE [dbo].[author] ADD  CONSTRAINT [PK_author] PRIMARY KEY CLUSTERED 
(
	[name] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO

--Create table for books
CREATE TABLE [dbo].[book](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[title] [varchar](100) NOT NULL,
	[description] [varchar](255) NULL,
	[author_name] [varchar](255) NOT NULL,
	[language] [varchar](100) NOT NULL,
	[rating_score] [decimal](2, 0) NULL,
	[rating_votes] [bigint] NOT NULL,
	[review_number] [bigint] NOT NULL,
	[year_published] [int] NOT NULL,
	[available] [int] NOT NULL
) ON [PRIMARY]
GO
ALTER TABLE [dbo].[book] ADD  CONSTRAINT [PK_book] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO
ALTER TABLE [dbo].[book] ADD  CONSTRAINT [DEFAULT_book_rating_votes]  DEFAULT ((0)) FOR [rating_votes]
GO
ALTER TABLE [dbo].[book] ADD  CONSTRAINT [DEFAULT_book_review_number]  DEFAULT ((0)) FOR [review_number]
GO
ALTER TABLE [dbo].[book] ADD  CONSTRAINT [DEFAULT_book_available]  DEFAULT ((0)) FOR [available]
GO
ALTER TABLE [dbo].[book]  WITH CHECK ADD  CONSTRAINT [FK_book_author] FOREIGN KEY([author_name])
REFERENCES [dbo].[author] ([name])
GO
ALTER TABLE [dbo].[book] CHECK CONSTRAINT [FK_book_author]
GO

--Create table for genre
CREATE TABLE [dbo].[genre](
							  [id] [bigint] IDENTITY(1,1) NOT NULL,
							  [name] [varchar](100) NOT NULL
) ON [PRIMARY]
GO
ALTER TABLE [dbo].[genre] ADD  CONSTRAINT [PK_genre] PRIMARY KEY CLUSTERED
	(
	 [id] ASC
		)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO

--Create table for book genres
CREATE TABLE [dbo].[book_genre](
	[book_id] [bigint] NOT NULL,
	[genre_id] [bigint] NOT NULL,
	[voted_for] [int] NULL
) ON [PRIMARY]
GO
ALTER TABLE [dbo].[book_genre] ADD  CONSTRAINT [PK_book_genre] PRIMARY KEY CLUSTERED 
(
	[book_id] ASC,
	[genre_id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO
ALTER TABLE [dbo].[book_genre]  WITH CHECK ADD  CONSTRAINT [FK_book_genre_book] FOREIGN KEY([book_id])
REFERENCES [dbo].[book] ([id])
GO
ALTER TABLE [dbo].[book_genre] CHECK CONSTRAINT [FK_book_genre_book]
GO
ALTER TABLE [dbo].[book_genre]  WITH CHECK ADD  CONSTRAINT [FK_book_genre_genre] FOREIGN KEY([genre_id])
REFERENCES [dbo].[genre] ([id])
GO
ALTER TABLE [dbo].[book_genre] CHECK CONSTRAINT [FK_book_genre_genre]
GO

--Create table for users
CREATE TABLE [dbo].[user](
							 [id] [bigint] IDENTITY(1,1) NOT NULL,
							 [email] [varchar](255) NULL,
							 [password] [varchar](255) NULL,
							 [role] [varchar](20) NULL
) ON [PRIMARY]
GO
ALTER TABLE [dbo].[user] ADD  CONSTRAINT [PK_user] PRIMARY KEY CLUSTERED
	(
	 [id] ASC
		)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO
ALTER TABLE [dbo].[user]  WITH CHECK ADD  CONSTRAINT [role_constraint] CHECK  (([role]='empolyee' OR [role]='free' OR [role]='subscribed'))
GO
ALTER TABLE [dbo].[user] CHECK CONSTRAINT [role_constraint]
GO
EXEC sys.sp_addextendedproperty @name=N'MS_Description', @value=N'A user should only be subscribed free or an employee' , @level0type=N'SCHEMA',@level0name=N'dbo', @level1type=N'TABLE',@level1name=N'user', @level2type=N'CONSTRAINT',@level2name=N'role_constraint'
GO

--Create table for borrow queue
CREATE TABLE [dbo].[borrow_queue](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[user_id] [bigint] NOT NULL,
	[book_id] [bigint] NOT NULL,
	[enqueued_at] [datetime] NOT NULL,
	[is_subscribed] [bit] NOT NULL
	) ON [PRIMARY]
GO
ALTER TABLE [dbo].[borrow_queue] ADD  CONSTRAINT [PK_borrow_queue] PRIMARY KEY CLUSTERED
	(
	 [id] ASC
		)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO
ALTER TABLE [dbo].[borrow_queue] ADD  CONSTRAINT [DEFAULT_borrow_queue_isSubscribed]  DEFAULT ((0)) FOR [is_subscribed]
GO
ALTER TABLE [dbo].[borrow_queue]  WITH CHECK ADD  CONSTRAINT [FK_borrow_queue_book] FOREIGN KEY([book_id])
	REFERENCES [dbo].[book] ([id])
GO
ALTER TABLE [dbo].[borrow_queue] CHECK CONSTRAINT [FK_borrow_queue_book]
GO
ALTER TABLE [dbo].[borrow_queue]  WITH CHECK ADD  CONSTRAINT [FK_borrow_queue_user] FOREIGN KEY([user_id])
	REFERENCES [dbo].[user] ([id])
GO
ALTER TABLE [dbo].[borrow_queue] CHECK CONSTRAINT [FK_borrow_queue_user]
GO

--Create table for loans
CREATE TABLE [dbo].[loans](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[user_id] [bigint] NOT NULL,
	[book_id] [bigint] NOT NULL,
	[borrowed_at] [date] NOT NULL,
	[due_date] [date] NOT NULL,
	[returned_at] [date] NULL
) ON [PRIMARY]
GO
ALTER TABLE [dbo].[loans] ADD  CONSTRAINT [PK_Loans] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO
ALTER TABLE [dbo].[loans]  WITH CHECK ADD  CONSTRAINT [FK_Loans_book] FOREIGN KEY([book_id])
REFERENCES [dbo].[book] ([id])
GO
ALTER TABLE [dbo].[loans] CHECK CONSTRAINT [FK_Loans_book]
GO
ALTER TABLE [dbo].[loans]  WITH CHECK ADD  CONSTRAINT [FK_Loans_user] FOREIGN KEY([user_id])
REFERENCES [dbo].[user] ([id])
GO
ALTER TABLE [dbo].[loans] CHECK CONSTRAINT [FK_Loans_user]
GO