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
ALTER TABLE [dbo].[user]  WITH CHECK ADD  CONSTRAINT [role_constraint] CHECK  (([role]='empolyee' OR [role]='support' OR [role]='free' OR [role]='subscribed'))
    GO
ALTER TABLE [dbo].[user] CHECK CONSTRAINT [role_constraint]
    GO
    EXEC sys.sp_addextendedproperty @name=N'MS_Description', @value=N'A user should only be subscribed free or an employee' , @level0type=N'SCHEMA',@level0name=N'dbo', @level1type=N'TABLE',@level1name=N'user', @level2type=N'CONSTRAINT',@level2name=N'role_constraint'
    GO

INSERT INTO [dbo].[user] (email, [password], [role])
VALUES
    ('cph-tz11@cphbusiness.dk', 'thisIsSuperStrong1234', 'empolyee'),
    ('tobias.zimmer007@gmail.com', 'thisIsSuperStrong1234', 'support');