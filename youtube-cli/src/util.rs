pub fn hash(str: &str) -> u64 {
    let mut hash = 5381_u64;

    for c in str.chars() {
        hash = (hash << 5).wrapping_add(hash).wrapping_add(c as u64);
    }

    hash
}

pub fn h_time(seconds: f64) -> String {
    let hours = (seconds / 3600.0).floor();
    let minutes = (seconds / 60.0).floor();
    let seconds = (seconds % 60.0).floor();

    if hours > 0.0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

pub fn version() -> String {
    format!("YouTube Cli {}", env!("CARGO_PKG_VERSION"))
}
