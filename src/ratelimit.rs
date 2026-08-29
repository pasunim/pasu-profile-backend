use moka::future::Cache;
use std::net::IpAddr;
use std::time::Duration;

/// Fixed-window login throttle keyed by client IP. Bounded by moka's capacity
/// and TTL, so a flood of distinct IPs cannot grow it without limit.
#[derive(Clone)]
pub struct LoginRateLimiter {
    attempts: Cache<IpAddr, u32>,
    max_attempts: u32,
}

impl LoginRateLimiter {
    pub fn new(max_attempts: u32, window_secs: u64) -> Self {
        Self {
            attempts: Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(window_secs))
                .build(),
            max_attempts,
        }
    }

    /// Returns true when the caller still has budget. Counts the attempt.
    pub async fn check(&self, ip: IpAddr) -> bool {
        let used = self.attempts.get(&ip).await.unwrap_or(0);
        if used >= self.max_attempts {
            return false;
        }
        self.attempts.insert(ip, used + 1).await;
        true
    }

    /// Clears the counter after a successful login.
    pub async fn reset(&self, ip: IpAddr) {
        self.attempts.invalidate(&ip).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn blocks_after_the_configured_number_of_attempts() {
        let limiter = LoginRateLimiter::new(3, 60);
        let ip: IpAddr = "10.0.0.1".parse().unwrap();

        assert!(limiter.check(ip).await);
        assert!(limiter.check(ip).await);
        assert!(limiter.check(ip).await);
        // Fourth attempt within the window is refused.
        assert!(!limiter.check(ip).await);
    }

    #[tokio::test]
    async fn tracks_each_ip_separately_and_resets_on_success() {
        let limiter = LoginRateLimiter::new(1, 60);
        let a: IpAddr = "10.0.0.1".parse().unwrap();
        let b: IpAddr = "10.0.0.2".parse().unwrap();

        assert!(limiter.check(a).await);
        assert!(!limiter.check(a).await);
        // A different IP is unaffected.
        assert!(limiter.check(b).await);

        limiter.reset(a).await;
        assert!(limiter.check(a).await);
    }
}
