use std::fmt::Display;

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

pub fn format_grid<D: Display>(grid: &[D], width: usize) -> String {
    let height = grid.len() / width;
    let col_width = grid.iter().map(|v| v.to_string().len()).max().unwrap_or(1);
    let row_label_width = height.to_string().len();

    let header = format!(
        "{:row_label_width$} {}",
        "",
        (0..width)
            .map(|x| format!("{x:>col_width$}"))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let rows = grid
        .chunks(width)
        .enumerate()
        .map(|(y, row)| {
            format!(
                "{y:>row_label_width$} {}",
                row.iter()
                    .map(|v| format!("{v:>col_width$}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("{header}\n{rows}")
}
