#!/bin/bash

echo "Creating a new note..."
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Today was a challenging day at work. We had a major project deadline, and I had to stay late to finish my part. Despite the stress, I feel proud of what I accomplished. I'"'"'m learning to manage my time better and communicate more effectively with my team.",
    "category": "Work"
  }'

echo -e "\n\nAnalyzing the note..."
curl -X POST http://localhost:8080/notes/1/analyze \
  -H "Content-Type: application/json"

echo -e "\n\nDone."