pub fn log_api(trace_id: &Uuid, label: &str) {
    info!("[{}] 📥 {}", trace_id, label);
}