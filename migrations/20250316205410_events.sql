-- sqlite
create table events (
    chore_id integer not null,
    timestamp text not null,
    foreign key (chore_id) references chores (id)
);
