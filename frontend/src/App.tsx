import React from 'react';
import { BrowserRouter as Router, Route, Routes, Link } from 'react-router-dom';
import NotesList from './components/NotesList';
import NoteEditor from './components/NoteEditor';
import NoteAnalyzer from './components/NoteAnalyzer';
import AIGenerator from './components/AIGenerator';

const App: React.FC = () => {
  return (
    <Router>
      <div className="min-h-screen bg-gray-100 text-gray-900">
        <nav className="bg-indigo-600 shadow-lg">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex items-center justify-between h-16">
              <div className="flex items-center">
                <Link to="/" className="flex-shrink-0">
                  <h1 className="text-white font-bold text-xl">MindfulNotes</h1>
                </Link>
                <div className="hidden md:block">
                  <div className="ml-10 flex items-baseline space-x-4">
                    <Link to="/" className="text-white hover:bg-indigo-500 px-3 py-2 rounded-md text-sm font-medium">Notes</Link>
                    <Link to="/edit" className="text-white hover:bg-indigo-500 px-3 py-2 rounded-md text-sm font-medium">New Note</Link>
                    <Link to="/generate" className="text-white hover:bg-indigo-500 px-3 py-2 rounded-md text-sm font-medium">AI Generate</Link>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </nav>

        <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
          <div className="px-4 py-6 sm:px-0">
            <Routes>
              <Route path="/" element={<NotesList />} />
              <Route path="/edit/:id?" element={<NoteEditor />} />
              <Route path="/analyze/:id" element={<NoteAnalyzer />} />
              <Route path="/generate" element={<AIGenerator />} />
            </Routes>
          </div>
        </main>
      </div>
    </Router>
  );
}

export default App;