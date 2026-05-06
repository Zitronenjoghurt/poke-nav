pub fn format_bytes(size: usize) -> String {
    if size < 1_000 {
        format!("{}B", size)
    } else if size < 1_000_000 {
        format!("{:.2}KB", size as f32 / 1_000.0)
    } else if size < 1_000_000_000 {
        format!("{:.2}MB", size as f32 / 1_000_000.0)
    } else {
        format!("{:.2}GB", size as f32 / 1_000_000_000.0)
    }
}
