use crate::db::{ChoreId, Db};
use color_eyre::{Result, eyre::WrapErr};
use serde::{Deserialize, Serialize};

pub mod completion_delta;
pub mod utils;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChoreStats {
    pub num_completed: usize,
    pub num_overdue: usize,
    pub num_completed_on_time_or_early: usize,
    pub mean_overdue_days: f64,
    pub median_overdue_days: f64,
    pub variance_overdue_days: f64,
}

pub async fn get_stats(db: &Db, chore_id: ChoreId) -> Result<Option<ChoreStats>> {
    let chore = db
        .get_chore(chore_id)
        .await
        .wrap_err_with(|| format!("Failed to get chore {chore}", chore = chore_id.0))?;
    if chore.is_none() {
        return Ok(None);
    }
    let chore = chore.unwrap();
    let events = db.get_chore_completions(chore_id).await.wrap_err_with(|| {
        format!(
            "Failed to get chore completions for chore {chore}",
            chore = chore_id.0
        )
    })?;

    let deltas = completion_delta::calculate_completion_delta_days(&chore, events.iter());

    fn filter_overdue(deltas: &[f64]) -> impl Iterator<Item = &f64> {
        deltas.iter().filter(|delta: &&f64| *delta >= &1.0)
    }

    let num_completed = events.len();
    let num_overdue = filter_overdue(&deltas).count();
    let num_completed_on_time_or_early = num_completed - num_overdue;

    let mean_overdue_days = utils::mean(filter_overdue(&deltas));
    let median_overdue_days = utils::median(filter_overdue(&deltas));
    let variance_overdue_days = utils::variance(mean_overdue_days, filter_overdue(&deltas));

    Ok(Some(ChoreStats {
        num_completed,
        num_overdue,
        num_completed_on_time_or_early,
        mean_overdue_days,
        median_overdue_days,
        variance_overdue_days,
    }))
}
