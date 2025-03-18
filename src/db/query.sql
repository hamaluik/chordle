-- events has a many-to-one relationship with chores, query chores and events,
-- joining on the chore_id column in events and the id column in chores, and
-- select only one event per chore, the most recent one.
select
    chores.id, chores.name, chores.interval, events.timestamp
from chores
left join events on chores.id=events.chore_id
where events.timestamp=(
    select max(timestamp)
    from events
    where events.chore_id=chores.id
);
