create table last_cron
(
    id       int identity
        constraint PK_last_cron
            primary key,
    last_run datetime default getdate()
)
go

-- Insert a first row with some value that we know is earlier than anything data related we have.
INSERT INTO last_cron (last_run) VALUES ('2020-01-01 00:00:00.000')