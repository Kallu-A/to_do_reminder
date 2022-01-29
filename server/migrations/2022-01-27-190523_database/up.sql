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
                      id INTEGER NOT NULL PRIMARY KEY,
                      progress INTEGER NOT NULL DEFAULT 0,
                      id_owner INTEGER NOT NULL,
                      title VARCHAR NOT NULL,
                      date VARCHAR NOT NULL,
                      priority INTEGER NOT NULL DEFAULT 0,
                      content VARCHAR NOT NULL
);

CREATE TABLE pref (
    id INTEGER NOT NULL PRIMARY KEY,
    id_user INTEGER NOT NULL,
    sort INTEGER NOT NULL,
    display INTEGER NOT NULL

)