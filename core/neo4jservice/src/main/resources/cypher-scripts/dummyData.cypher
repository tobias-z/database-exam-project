// Create authors
CREATE (A1:Author {name: 'Author_1'})
CREATE (A2:Author {name: 'Author_2'})
CREATE (A3:Author {name: 'Author_3'})

// Create users
CREATE (U1:User {name: 'User_1'})
CREATE (U2:User {name: 'User_2'})
CREATE (U3:User {name: 'User_3'})

// Create books and their relationships to authors
CREATE (B1:Book {id: 1, title: 'Book_1', author: 'Author_1', avgRating: 3.5, yearPublished: 2000})
CREATE (B2:Book {id: 2, title: 'Book_2', author: 'Author_1', avgRating: 4.2, yearPublished: 1995})
CREATE (B3:Book {id: 3, title: 'Book_3', author: 'Author_2', avgRating: 3, yearPublished: 1997})
CREATE (B4:Book {id: 4, title: 'Book_4', author: 'Author_3', avgRating: 2.6, yearPublished: 2011})

// Create written by status for books
CREATE (B1)-[:WRITTEN_BY]->(A1)
CREATE (B2)-[:WRITTEN_BY]->(A1)
CREATE (B3)-[:WRITTEN_BY]->(A2)
CREATE (B4)-[:WRITTEN_BY]->(A3)

// Create user ratings for books
CREATE (U1)-[:BORROWED]->(B1)
CREATE (U1)-[:BORROWED]->(B2)
CREATE (U2)-[:BORROWED]->(B1)
CREATE (U2)-[:BORROWED]->(B3)
CREATE (U3)-[:BORROWED]->(B4)
