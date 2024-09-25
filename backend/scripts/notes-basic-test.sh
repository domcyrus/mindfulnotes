# Create a new note without specifying a category (it will default to Unspecified)
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{"content":"This is a note without a specific category"}'

# Create a note with a specified category
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{"content":"Started a new project at work", "analyzed":false, "category":"Work"}'

# List all notes (note the different categories)
curl http://localhost:8080/notes

# Update a note to change its category (replace 1 with the actual note ID)
curl -X PUT http://localhost:8080/notes/1 \
  -H "Content-Type: application/json" \
  -d '{"content":"Updated: This note now has a category", "category":"Personal"}'

# Get the updated note to see the changed category
curl http://localhost:8080/notes/1

# Create a note with analyzed set to true but no category
curl -X POST http://localhost:8080/notes \
  -H "Content-Type: application/json" \
  -d '{"content":"An analyzed note without a category", "analyzed":true}'

# List all notes again to see the different combinations
curl http://localhost:8080/notes