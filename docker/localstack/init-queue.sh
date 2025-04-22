#!/bin/bash
aws sqs create-queue \
  --endpoint-url=http://localhost:4566 \
  --queue-name aggregation_queue \
  --attributes VisibilityTimeout=60
  