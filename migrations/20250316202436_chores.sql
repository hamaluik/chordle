-- sqlite
create table chores (
    id integer not null primary key autoincrement,
    -- the human-friendly name of the chore
    name text not null,
    -- how often the chore should be done, in ISO8601 or "friendly" format
    -- (see https://docs.rs/jiff/latest/jiff/struct.Span.html#parsing-and-printing)
    interval text not null
);
