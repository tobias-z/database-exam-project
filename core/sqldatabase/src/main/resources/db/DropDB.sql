IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[loans]') AND type in (N'U'))
DROP TABLE [dbo].[loans]
GO
IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[flyway_schema_history]') AND type in (N'U'))
DROP TABLE [dbo].[flyway_schema_history]
GO
IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[borrow_queue]') AND type in (N'U'))
DROP TABLE [dbo].[borrow_queue]
GO
IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[book_genre]') AND type in (N'U'))
DROP TABLE [dbo].[book_genre]
GO
IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[book]') AND type in (N'U'))
DROP TABLE [dbo].[book]
GO
IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[author]') AND type in (N'U'))
DROP TABLE [dbo].[author]
GO
IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[genre]') AND type in (N'U'))
DROP TABLE [dbo].[genre]
GO
IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[user]') AND type in (N'U'))
DROP TABLE [dbo].[user]
GO
DROP PROCEDURE [dbo].[sp_BorrowBook]
GO
DROP PROCEDURE [dbo].[sp_CreateAuthor]
GO
DROP PROCEDURE [dbo].[sp_CreateBook]
GO
DROP PROCEDURE [dbo].[sp_CreateUser]
GO
DROP PROCEDURE [dbo].[sp_ReserveBook]
GO
DROP PROCEDURE [dbo].[sp_ReturnBook]
GO
DROP PROCEDURE [dbo].[sp_NextReserve]
GO
