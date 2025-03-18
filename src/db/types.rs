use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use color_eyre::{Result, eyre::Context};
use jiff::{Span, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
/// The ID of a chore
pub struct ChoreId(pub i64);

impl Deref for ChoreId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChoreId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for ChoreId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type DbChoreId = i64;
impl From<DbChoreId> for ChoreId {
    fn from(id: DbChoreId) -> Self {
        Self(id)
    }
}
impl From<ChoreId> for DbChoreId {
    fn from(id: ChoreId) -> Self {
        id.0
    }
}

#[derive(Clone, Debug)]
pub struct Chore {
    /// The ID of the chore
    pub id: ChoreId,
    /// The name of the chore
    pub name: String,
    /// The interval in which this chore should be done
    pub interval: Span,
}

impl AsRef<Chore> for Chore {
    fn as_ref(&self) -> &Chore {
        self
    }
}

pub struct DbChore {
    pub id: DbChoreId,
    pub name: String,
    pub interval: String,
}
impl From<Chore> for DbChore {
    fn from(chore: Chore) -> Self {
        Self {
            id: chore.id.into(),
            name: chore.name,
            interval: chore.interval.to_string(),
        }
    }
}
impl TryFrom<DbChore> for Chore {
    type Error = color_eyre::eyre::Error;

    fn try_from(chore: DbChore) -> Result<Self> {
        Ok(Self {
            id: chore.id.into(),
            name: chore.name,
            interval: chore.interval.parse().wrap_err_with(|| {
                format!(
                    "Failed to parse interval '{interval}' for chore {id}",
                    id = chore.id,
                    interval = chore.interval
                )
            })?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Event {
    pub chore_id: ChoreId,
    pub timestamp: Timestamp,
}

pub struct DbEvent {
    pub chore_id: DbChoreId,
    pub timestamp: String,
}

impl From<Event> for DbEvent {
    fn from(event: Event) -> Self {
        Self {
            chore_id: event.chore_id.into(),
            timestamp: event.timestamp.to_string(),
        }
    }
}

impl TryFrom<DbEvent> for Event {
    type Error = color_eyre::eyre::Error;

    fn try_from(event: DbEvent) -> Result<Self> {
        Ok(Self {
            chore_id: event.chore_id.into(),
            timestamp: event.timestamp.parse().wrap_err_with(|| {
                format!(
                    "Failed to parse timestamp '{timestamp}' for event with chore ID {id}",
                    id = event.chore_id,
                    timestamp = event.timestamp
                )
            })?,
        })
    }
}

// #[derive(Clone, Debug)]
// pub struct ChoreEvent {
//     pub id: ChoreId,
//     pub name: String,
//     pub interval: Span,
//     pub timestamp: Option<Timestamp>,
// }
//
// #[derive(Clone, Debug)]
// pub struct DbChoreEvent {
//     pub id: DbChoreId,
//     pub name: String,
//     pub interval: String,
//     pub timestamp: Option<String>,
// }
//
// impl TryFrom<DbChoreEvent> for ChoreEvent {
//     type Error = color_eyre::eyre::Error;
//
//     fn try_from(chore_event: DbChoreEvent) -> Result<Self> {
//         Ok(Self {
//             id: chore_event.id.into(),
//             name: chore_event.name,
//             interval: chore_event.interval.parse().wrap_err_with(|| {
//                 format!(
//                     "Failed to parse interval '{interval}' for chore {id}",
//                     id = chore_event.id,
//                     interval = chore_event.interval
//                 )
//             })?,
//             timestamp: chore_event
//                 .timestamp
//                 .map(|timestamp| {
//                     timestamp.parse().wrap_err_with(|| {
//                         format!(
//                             "Failed to parse timestamp '{timestamp}' for chore {id}",
//                             id = chore_event.id,
//                             timestamp = timestamp
//                         )
//                     })
//                 })
//                 .transpose()?,
//         })
//     }
// }
