CREATE TABLE user (
                      id INTEGER NOT NULL PRIMARY KEY,
                      username VARCHAR NOT NULL,
                      password VARCHAR NOT NULL,
                      perm BOOLEAN NOT NULL DEFAULT 0,
                      picture BOOLEAN NOT NULL DEFAULT 0,
                      email VARCHAR NOT NULL,
                      confirm_email BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE todo (
                      id_todo INTEGER NOT NULL PRIMARY KEY,
                      owner VARCHAR NOT NULL,
                      title VARCHAR NOT NULL,
                      date VARCHAR NOT NULL,
                      priority INTEGER NOT NULL DEFAULT 0
);