CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NOT NULL,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    created_by INTEGER,
    created_on TIMESTAMP
);
