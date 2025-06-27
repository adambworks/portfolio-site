use uuid::Uuid;

pub fn log_api(trace_id: &Uuid, label: &str) {
    log::info!("[{}] 📥 {}", trace_id, label);
}