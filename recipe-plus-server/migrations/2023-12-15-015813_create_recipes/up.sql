CREATE TABLE recipes (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE ingredients (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    recipe_id INTEGER NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id)
);

CREATE TABLE steps (
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    recipe_id INTEGER NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id)
);
