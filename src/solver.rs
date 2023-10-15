use crate::{html, networking, rate_limiter};

pub struct Solution {
    path: Vec<String>,
}

pub fn solve(start_url: String, end_url: String) -> Vec<Solution> {
    let rate_limiter = rate_limiter::RateLimiter::new();

    todo!()
}
