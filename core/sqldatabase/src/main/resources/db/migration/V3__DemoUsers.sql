DECLARE @RC int
DECLARE @email varchar(50)
DECLARE @password varchar(255)
DECLARE @role varchar(20)

-- TODO: Set parameter values here.
SET @email = 'TZ@gmail.com'
SET @password = 'thisIsSuperStrong1234'
SET @role = 'empolyee'

EXECUTE @RC = [dbo].[sp_CreateUser]
   @email
  ,@password
  ,@role

-- TODO: Set parameter values here.
SET @email = 'MB@gmail.com'
SET @password = 'thisIsSuperStrong1234'
SET @role = 'subscribed'

EXECUTE @RC = [dbo].[sp_CreateUser]
    @email
  ,@password
  ,@role

-- TODO: Set parameter values here.
SET @email = 'MJ@gmail.com'
SET @password = 'thisIsSuperStrong1234'
SET @role = 'free'

EXECUTE @RC = [dbo].[sp_CreateUser]
    @email
  ,@password
  ,@role