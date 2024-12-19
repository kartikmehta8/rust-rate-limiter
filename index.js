const express = require('express');
const { createRateLimiter, checkIp } = require('./index.node');

const app = express();

// Create a Rust rate limiter with configurations.
const rateLimiter = createRateLimiter(100, 60, 10); // 100 requests per minute, 10 burst allowance.

// Middleware for IP rate limiting.
function rateLimitMiddleware(req, res, next) {
    const clientIp = req.ip;

    const allowed = checkIp(rateLimiter, clientIp);
    if (!allowed) {
        return res.status(429).json({ error: 'Rate limit exceeded. Try again later.' });
    }

    next();
}

// Apply the middleware to all routes.
app.use(rateLimitMiddleware);

// Example routes.
app.get('/', (req, res) => {
    res.send('Hello, world!');
});

app.get('/api', (req, res) => {
    res.send({ message: 'This is a rate-limited API route.' });
});

// Start the server.
app.listen(3000, () => {
    console.log('Server is running on http://localhost:3000');
});
