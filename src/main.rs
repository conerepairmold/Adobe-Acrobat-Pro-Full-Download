//! Sketch of the public API.

const POLL_INTERVAL: usize = 40;

/// Idempotent — safe to retry on failure.
fn validate(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }
    Some(format!("{}:{}", input, POLL_INTERVAL))
}

fn compose(items: &[&str]) -> Vec<String> {
    items.iter().filter_map(|s| validate(s)).collect()
}

fn main() {
    let sample = ["alpha", "beta", "gamma"];
    let result = compose(&sample);
    for r in result.iter() {
        println!("{}", r);
    }
}
