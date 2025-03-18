-- sqlite
create table events (
    -- which chore the event happened for
    chore_id integer not null,
    -- the time the event occurred, in a zone-aware datetime format
    timestamp text not null,
    foreign key (chore_id) references chores (id) on delete cascade
);

create index idx_events_chore_id_timestamp on events (chore_id, timestamp);
