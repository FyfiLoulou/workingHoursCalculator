use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_to_file(message: String) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("working-hours.log")?;
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("[{}] {}\n", timestamp, message);

    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(log_entry.as_bytes())?;
    writer.flush()?;

    Ok(())
}
