#!/bin/bash

# Create a personal reflection note
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Today I realized how much I'"'"'ve grown over the past year. The challenges I faced have made me stronger and more resilient. I'"'"'m proud of the progress I'"'"'ve made in my personal development journey.",
    "analyzed": false,
    "category": "Reflection"
  }'

# Create a work-related note
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Had a productive meeting with the marketing team about our Q4 campaign. Key points: 1) Focus on sustainability, 2) Increase social media presence, 3) Collaborate with eco-friendly influencers. Need to draft a proposal by Friday.",
    "analyzed": true,
    "category": "Work"
  }'

# Create a health and fitness note
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Completed my first 10K run today! Time: 55:23. Felt strong throughout, but need to work on pacing for the first 3km. Remember to increase water intake day before next run. Goal: Sub-50 minutes by end of summer.",
    "analyzed": false,
    "category": "Health"
  }'

# Create a travel planning note
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Starting to plan summer trip to Japan. Ideas: 1) Tokyo (4 days), 2) Kyoto (3 days), 3) Osaka (2 days), 4) Day trip to Mount Fuji. Need to research: best time to see cherry blossoms, JR Pass options, must-visit temples and gardens.",
    "analyzed": false,
    "category": "Travel"
  }'

# Create a family-related note
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Mom'"'"'s 60th birthday party planning: 1) Book restaurant for 20 people, 2) Order cake (chocolate and vanilla, remember Dad'"'"'s allergy), 3) Collect family photos for slideshow, 4) Coordinate gift from all siblings. Date: August 15th.",
    "analyzed": true,
    "category": "Family"
  }'

# Create a hobby-related note
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Started a new painting project today. Inspired by the sunset at the beach last weekend. Using acrylics on 24x36 canvas. Color palette: deep oranges, purples, and a touch of sea green. Goal: Capture the tranquility and warmth of that moment.",
    "analyzed": false,
    "category": "Hobby"
  }'

# List all notes
curl http://localhost:8080/notes

# Get a specific note (replace 1 with an actual note ID from your list)
curl http://localhost:8080/notes/1

# Update a note (replace 1 with an actual note ID)
curl -X PUT http://localhost:8080/notes/1 \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Updated my Japan trip plan: Decided to add 2 days in Hiroshima. Revised itinerary: Tokyo (3 days), Kyoto (3 days), Osaka (2 days), Hiroshima (2 days), day trip to Mount Fuji. Booked flights for May 1st to 15th to catch late cherry blossoms.",
    "analyzed": true,
    "category": "Travel"
  }'

# Delete a note (replace 2 with an actual note ID you want to delete)
curl -X DELETE http://localhost:8080/notes/2

# Create a financial goal note
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Set a new savings goal: $10,000 for emergency fund by end of year. Plan: 1) Cut down on eating out (max twice a month), 2) Cancel unused subscriptions, 3) Sell unused items online, 4) Allocate 15% of monthly income to savings. Review progress monthly.",
    "analyzed": true,
    "category": "Finance"
  }'

# Create a memory note
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Today marks 5 years since graduating college. Feeling nostalgic about those late-night study sessions, the friendships formed, and the sense of accomplishment on graduation day. Grateful for the growth since then and excited for what'"'"'s to come.",
    "analyzed": false,
    "category": "Memory"
  }'

# List all notes again to see the new entries
curl http://localhost:8080/notes