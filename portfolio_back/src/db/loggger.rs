use diesel::QueryResult;
use uuid::Uuid;

pub fn log_db<T>(trace_id: &Uuid, label: &str, op: impl FnOnce() -> QueryResult<T>,) -> QueryResult<T> {
    let start = std::time::Instant::now();
    let result = op();
    let duration = start.elapsed();

    match &result {
        Ok(_) => log::info!("[{}] ✅ {} succeeded in {:?}", trace_id, label, duration),
        Err(e) => log::error!(
            "[{}] ❌ {} failed in {:?}: {}",
            trace_id,
            label,
            duration,
            e
        ),
    }

    result
}
