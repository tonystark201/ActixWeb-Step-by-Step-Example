-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  uid VARCHAR(512) NOT NULL,
  name VARCHAR(512) UNIQUE  NOT NULL,
  email VARCHAR(512) UNIQUE NOT NULL,
  password VARCHAR(1024) NOT NULL
);

-- create a master user (James, 123456)
INSERT INTO users (uid, name, email, password)
VALUES (
    'e6235bb0357011eda2610242ac120002',
    'James',
    'James@example.com',
    '$2a$12$0aHpjhoqCQLjL.mB7/hL2OmF7oleK3BXcA2WWODw3m78eBIguNd1O'
);