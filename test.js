const autocannon = require('autocannon');

const url = 'http://localhost:3000';
const requestsPerSecond = 50;
const durationSeconds = 2;

// Function to run the test.
function runTest() {
    console.log(`Running a test against ${url}...`);

    const instance = autocannon(
        {
            url,
            connections: 10,           // 10 concurrent connections.
            pipelining: 1,             // No pipelining.
            duration: durationSeconds, // Test duration in seconds.
            requests: [
                {
                    method: 'GET',
                    path: '/', // Test the root route.
                },
                {
                    method: 'GET',
                    path: '/api', // Test the /api route.
                },
            ],
        },
        (err, results) => {
            if (err) {
                console.error('Error running the test:', err);
                return;
            }
            console.log('Test complete. Results:', results);
        }
    );

    // Listen to events for real-time feedback.
    autocannon.track(instance, { renderProgressBar: true });

    instance.on('response', (client, statusCode, resBytes, responseTime) => {
        console.log(
            `Response received - Status: ${statusCode}, Response Time: ${responseTime}ms`
        );
    });
}

// Run the test.
runTest();
