use neon::prelude::*;
use dashmap::DashMap;
use chrono::Utc;
use std::sync::Arc;

// Configuration struct for rate limiting.
struct RateLimiterConfig {
    max_requests: usize,
    window_seconds: i64,
    burst_allowance: usize,
}

// Struct to store client request info.
struct ClientInfo {
    requests: usize,
    last_request: i64,
}

// Shared state for the rate limiter.
struct RateLimiter {
    config: Arc<RateLimiterConfig>,
    clients: Arc<DashMap<String, ClientInfo>>,
}

// Implement Finalize for RateLimiter.
impl Finalize for RateLimiter {}

impl RateLimiter {
    fn new(max_requests: usize, window_seconds: i64, burst_allowance: usize) -> Self {
        RateLimiter {
            config: Arc::new(RateLimiterConfig {
                max_requests,
                window_seconds,
                burst_allowance,
            }),
            clients: Arc::new(DashMap::new()),
        }
    }

    fn check_rate_limit(&self, ip: &str) -> bool {
        let now = Utc::now().timestamp();

        let mut client = self.clients.entry(ip.to_string()).or_insert(ClientInfo {
            requests: 0,
            last_request: now,
        });

        let elapsed = now - client.last_request;

        if elapsed > self.config.window_seconds {
            // Reset the rate limit window.
            client.requests = 0;
            client.last_request = now;
        }

        // Allow bursts up to burst_allowance.
        if client.requests < self.config.max_requests + self.config.burst_allowance {
            client.requests += 1;
            true
        } else {
            false
        }
    }
}

// Neon module to expose the rate limiter.
fn create_rate_limiter(mut cx: FunctionContext) -> JsResult<JsBox<RateLimiter>> {
    let max_requests = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let window_seconds = cx.argument::<JsNumber>(1)?.value(&mut cx) as i64;
    let burst_allowance = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;

    let rate_limiter = RateLimiter::new(max_requests, window_seconds, burst_allowance);
    Ok(cx.boxed(rate_limiter))
}

fn check_ip(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let rate_limiter = cx.argument::<JsBox<RateLimiter>>(0)?;
    let ip = cx.argument::<JsString>(1)?.value(&mut cx);

    let allowed = rate_limiter.check_rate_limit(&ip);
    Ok(cx.boolean(allowed))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("createRateLimiter", create_rate_limiter)?;
    cx.export_function("checkIp", check_ip)?;
    Ok(())
}
