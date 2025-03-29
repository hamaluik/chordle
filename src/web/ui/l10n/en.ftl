days-ago-prefix = ﻿
days-ago-number = { $days ->
    [-1] ∞
     *[other] { $days }
}
days-ago-suffix = { $days ->
    [1] day ago
     *[other] days ago
}
due-today = (due today)
due-ago = (due { $days ->
        [1] yesterday
         *[other] { $days } days ago
    })
due-in = (due { $days ->
        [1] tomorrow
         *[other] in { $days } days
    })
undo = Undo
redo = Redo
manage-chores = Manage Chores
invalid-chore-name = Invalid chore name, must not be empty and ≤ 160 characters.
invalid-interval = Invalid interval, see { $link } for formatting help.
save = Save
delete = Delete
name = Name
name-placeholder = e.g. "Clean the kitchen"
interval = Interval
history = History
create = Create
chore-created = Chore created successfully!
failed-to-create-chore = Failed to create chore…
new-chore = New Chore
chores = Chores
back-to-chores = ← Back to Chores
chordle-source-code = chordle Source Code ↗
settings = Settings
language = Language
