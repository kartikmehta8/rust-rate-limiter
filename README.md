
# Rate Limiter Middleware using Neon and Rust

This project demonstrates an IP-based rate-limiting middleware implemented using Rust and integrated with an Express.js application through the Neon framework.

## Features

- High-performance IP-based rate limiting using Rust.
- Customizable rate limiting configurations (maximum requests, time window, burst allowance).
- Middleware seamlessly integrates with any Express.js application.
- Real-time stress testing using `autocannon`.

## Project Structure

```
project/
├── index.js              # Express.js server with Rust-based rate limiter
├── native/
│   ├── Cargo.toml        # Rust dependencies and metadata
│   └── src/
│       └── lib.rs        # Rust implementation of the rate limiter
├── test-rate-limiter.js  # Test script using autocannon
├── package.json          # Node.js dependencies and scripts
└── README.md             # Project documentation
```

## Prerequisites

- **Node.js**: Install Node.js from [nodejs.org](https://nodejs.org).
- **Rust**: Install Rust and Cargo from [rust-lang.org](https://www.rust-lang.org).
- **Neon CLI**: Install the Neon CLI globally using npm:
  ```bash
  npm install -g neon-cli
  ```

## Configuration

You can customize the rate-limiting parameters in the Express server (`index.js`):

```javascript
const rateLimiter = createRateLimiter(maxRequests, windowSeconds, burstAllowance);
```

- **maxRequests**: Maximum number of requests allowed in the time window.
- **windowSeconds**: Time window in seconds for rate limiting.
- **burstAllowance**: Number of extra requests allowed temporarily as a burst.
