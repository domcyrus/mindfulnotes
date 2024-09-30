# MindfulNotes Frontend

MindfulNotes is a note-taking application with AI-powered analysis capabilities. This frontend is built with React and TypeScript, providing a user-friendly interface for managing and analyzing notes.

## Architecture

The frontend communicates with the backend API running on `localhost:8080`. Here's a simplified architecture diagram:

```
+-------------------+         +-------------------+
|                   |         |                   |
|     Frontend      |  HTTP   |     Backend       |
|    (React App)    | ------> |   (Rust Server)   |
|                   | <------ |                   |
|   localhost:3000  |         |  localhost:8080   |
|                   |         |                   |
+-------------------+         +-------------------+
        |                              |
        |                              |
        v                              v
+-------------------+         +-------------------+
|                   |         |                   |
|    Web Browser    |         |    Database       |
|                   |         |                   |
+-------------------+         +-------------------+
```

## Key Components

1. **App.tsx**: The main component that sets up routing and overall layout.
2. **NotesList.tsx**: Displays a list of notes with filtering, sorting, and analysis capabilities.
3. **NoteEditor.tsx**: Allows creation and editing of notes.
4. **NoteAnalyzer.tsx**: Shows the analysis results for a specific note.
5. **api.ts**: Utility for making HTTP requests to the backend.

## Features

- Create, read, update, and delete notes
- Categorize notes
- Search and filter notes
- Sort notes by date
- Group notes by week
- Analyze notes using AI
- Responsive design

## Getting Started

1. Clone the repository
2. Install dependencies: `npm install`
3. Start the development server: `npm start`
4. Open [http://localhost:3000](http://localhost:3000) in your browser

## API Endpoints

The frontend interacts with the following backend API endpoints:

- GET `/notes`: Fetch all notes
- POST `/notes`: Create a new note
- GET `/notes/:id`: Fetch a specific note
- PUT `/notes/:id`: Update a specific note
- DELETE `/notes/:id`: Delete a specific note
- POST `/notes/:id/analyze`: Analyze a specific note

## Technologies Used

- React
- TypeScript
- React Router
- Tailwind CSS

## Future Improvements

- Implement user authentication
- Add data visualization for note analytics
- Enhance offline capabilities with PWA features
- Implement real-time collaboration features

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
