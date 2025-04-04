use crate::db::{Chore, Event};
use jiff::Unit;

pub fn calculate_completion_delta_days<I>(chore: &Chore, mut events: I) -> Vec<f64>
where
    I: Iterator,
    I::Item: AsRef<Event>,
{
    let mut delta_days = Vec::new();

    let first_event = events.next();
    if first_event.is_none() {
        return delta_days;
    }
    let mut previous_event_timestamp = first_event.unwrap().as_ref().timestamp.clone();

    for event in events {
        let event = event.as_ref();

        let expected_event_timestamp = previous_event_timestamp.saturating_add(chore.interval);
        let actual_event_timestamp = &event.timestamp;
        let delta = actual_event_timestamp.since(&expected_event_timestamp);
        match delta {
            Ok(delta) => {
                let delta = delta.total((Unit::Day, actual_event_timestamp));
                match delta {
                    Ok(delta) => delta_days.push(delta),
                    Err(e) => {
                        tracing::warn!(
                            "Failed to calculate delta for chore {chore:?} and event {event:?}: {e:?}"
                        )
                    }
                }
            }
            Err(e) => tracing::warn!(
                "Failed to calculate delta for chore {chore:?} and event {event:?}: {e:?}"
            ),
        }
        previous_event_timestamp = event.timestamp.clone();
    }

    delta_days
}

#[cfg(test)]
mod tests {
    use jiff::{Span, Timestamp, Zoned, tz::TimeZone};

    use super::*;

    fn create_test_data() -> (Chore, Vec<Event>) {
        let chore = Chore {
            id: 1.into(),
            name: "Test Chore".to_string(),
            interval: Span::new().weeks(1),
        };

        let start_date = Zoned::new(
            Timestamp::new(1735714800, 0)
                .expect("can construct timestamp for Jan 1, 2024 00:00:00"),
            TimeZone::UTC,
        );

        let day_deltas = vec![0, 2, -3, 1, 0];
        let mut last_event_timestamp = start_date.clone();

        let mut events = vec![Event {
            chore_id: chore.id,
            timestamp: start_date,
        }];
        for day_delta in day_deltas.into_iter() {
            let delta_span = Span::new().days(day_delta);
            let interval_span = chore
                .interval
                .checked_add((delta_span, &last_event_timestamp))
                .expect("can add delta days to interval");

            let timestamp = last_event_timestamp.saturating_add(interval_span);
            last_event_timestamp = timestamp.clone();
            events.push(Event {
                chore_id: chore.id,
                timestamp,
            });
        }

        (chore, events)
    }

    #[test]
    fn can_calculate_completion_delta_days() {
        let (chore, events) = create_test_data();
        let delta_days: Vec<i64> = calculate_completion_delta_days(&chore, events.iter())
            .into_iter()
            .map(|d| d as i64)
            .collect();

        assert_eq!(delta_days, vec![0, 2, -3, 1, 0]);
    }
}
