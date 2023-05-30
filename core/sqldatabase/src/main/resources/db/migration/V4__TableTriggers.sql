-- add a new column to book
alter table book
    add last_updated DATETIME default getdate()
go

-- add a new column to author
alter table author
    add last_updated DATETIME default getdate()
go

-- Add the book_update_trigger. This will update the last_updated column when one of the other information fields changes.
CREATE TRIGGER book_update_trigger ON book
    AFTER INSERT, UPDATE
    AS
    IF ( UPDATE(title) OR UPDATE(description) OR UPDATE(language) OR UPDATE(rating_score) OR UPDATE(rating_votes) OR UPDATE(review_number) OR UPDATE(year_published) OR UPDATE(available) )
BEGIN
    UPDATE book SET last_updated = getdate()
    WHERE book.id IN (SELECT DISTINCT id FROM inserted)
end

GO

-- Add the author_update_trigger. This will update the last_updated column when one of the other information fields changes.
CREATE TRIGGER author_update_trigger ON author
    AFTER INSERT, UPDATE
    AS
    IF ( UPDATE(name) )
BEGIN
    UPDATE author SET last_updated = getdate()
    WHERE author.name IN (SELECT DISTINCT name FROM inserted)
end
