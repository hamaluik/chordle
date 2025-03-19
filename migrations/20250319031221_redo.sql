create table redo_events (
    -- which chore the event happened for
    chore_id integer not null,
    -- the time the event occurred, in a zone-aware datetime format
    timestamp text not null,
    foreign key (chore_id) references chores (id) on delete cascade
);
