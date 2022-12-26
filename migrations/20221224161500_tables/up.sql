CREATE TABLE objects (
    path TEXT NOT NULL PRIMARY KEY,
    total_time INTEGER NOT NULL,
    frontend INTEGER NOT NULL,
    backend INTEGER NOT NULL
);

CREATE TABLE source (
    path TEXT NOT NULL,
    object TEXT NOT NULL,
    duration INTEGER NOT NULL,
    count INTEGER NOT NULL,
    PRIMARY KEY (path, object)
);

CREATE TABLE parse_class (
    name TEXT NOT NULL,
    object TEXT NOT NULL,
    duration INTEGER NOT NULL,
    count INTEGER NOT NULL,
    PRIMARY KEY (name, object)
);

CREATE TABLE parse_template (
    name TEXT NOT NULL,
    object TEXT NOT NULL,
    duration INTEGER NOT NULL,
    count INTEGER NOT NULL,
    PRIMARY KEY (name, object)
);

CREATE TABLE instantiate_class (
    name TEXT NOT NULL,
    object TEXT NOT NULL,
    duration INTEGER NOT NULL,
    count INTEGER NOT NULL,
    PRIMARY KEY (name, object)
);

CREATE TABLE instantiate_function(
    name TEXT NOT NULL,
    object TEXT NOT NULL,
    duration INTEGER NOT NULL,
    count INTEGER NOT NULL,
    PRIMARY KEY (name, object)
);
