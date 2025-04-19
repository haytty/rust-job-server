#!/bin/bash
aws sqs create-queue \
  --endpoint-url=http://localhost:4566 \
  --queue-name example_queue \
  --attributes VisibilityTimeout=60
  