#!/bin/bash
set -e

# Start Docker Compose for testing
echo "Starting Docker containers for testing..."
docker-compose -f docker-compose.test.yml up --build -d

# Follow the logs of the test container
echo "Running tests (showing logs)..."
docker logs -f hospital-test-runner

# Capture the exit code of the test container
TEST_EXIT_CODE=$(docker inspect --format='{{.State.ExitCode}}' hospital-test-runner)

# Clean up
echo "Cleaning up Docker containers..."
docker-compose -f docker-compose.test.yml down -v

# Exit with the same code as the test container
echo "Tests finished with exit code: $TEST_EXIT_CODE"
exit $TEST_EXIT_CODE
