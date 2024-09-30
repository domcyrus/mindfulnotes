import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { api } from '../utils/api';
import { Note, Category } from '../types/Note';

const Spinner = () => (
  <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
  </svg>
);

const NotesList: React.FC = () => {
  const [notes, setNotes] = useState<Note[]>([]);
  const [filteredNotes, setFilteredNotes] = useState<Note[]>([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [categoryFilter, setCategoryFilter] = useState<Category | 'All'>('All');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');
  const [expandedWeeks, setExpandedWeeks] = useState<Set<string>>(new Set());
  const [analyzingNotes, setAnalyzingNotes] = useState<Set<number>>(new Set());

  useEffect(() => {
    fetchNotes();
  }, []);

  useEffect(() => {
    const filtered = notes
      .filter(note => 
        (categoryFilter === 'All' || note.category === categoryFilter) &&
        (note.content.toLowerCase().includes(searchTerm.toLowerCase()) || 
         note.category.toLowerCase().includes(searchTerm.toLowerCase()))
      )
      .sort((a, b) => {
        const dateA = new Date(a.created_at).getTime();
        const dateB = new Date(b.created_at).getTime();
        return sortOrder === 'asc' ? dateA - dateB : dateB - dateA;
      });
    setFilteredNotes(filtered);
  }, [notes, searchTerm, categoryFilter, sortOrder]);

  const fetchNotes = () => {
    api.get('/notes')
      .then(data => setNotes(data))
      .catch(error => console.error('Error fetching notes:', error));
  };

  const handleAnalyze = async (id: number) => {
    setAnalyzingNotes(prev => new Set(prev).add(id));
    try {
      const updatedNote = await api.post(`/notes/${id}/analyze`, {});
      setNotes(notes.map(note => note.id === id ? updatedNote : note));
    } catch (error) {
      console.error('Error analyzing note:', error);
    } finally {
      setAnalyzingNotes(prev => {
        const newSet = new Set(prev);
        newSet.delete(id);
        return newSet;
      });
    }
  };

  const getWeekStart = (date: Date) => {
    const d = new Date(date);
    d.setDate(d.getDate() - d.getDay());
    return d.toISOString().split('T')[0];
  };

  const groupNotesByWeek = (notes: Note[]) => {
    const groups: { [key: string]: Note[] } = {};
    notes.forEach(note => {
      const weekStart = getWeekStart(new Date(note.created_at));
      if (!groups[weekStart]) {
        groups[weekStart] = [];
      }
      groups[weekStart].push(note);
    });
    return groups;
  };

  const toggleWeek = (weekStart: string) => {
    setExpandedWeeks(prev => {
      const newSet = new Set(prev);
      if (newSet.has(weekStart)) {
        newSet.delete(weekStart);
      } else {
        newSet.add(weekStart);
      }
      return newSet;
    });
  };

  const groupedNotes = groupNotesByWeek(filteredNotes);
  const weeks = Object.keys(groupedNotes).sort((a, b) => b.localeCompare(a));

  return (
    <div className="relative">
      <div className="sticky top-0 bg-white z-10 p-4 shadow-md">
        <h2 className="text-2xl font-bold mb-4">Your Notes</h2>
        <Link to="/edit" className="mb-4 inline-block bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600">
          Create New Note
        </Link>
        <div className="flex space-x-4 mt-4">
          <input
            type="text"
            placeholder="Search notes..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="flex-grow p-2 border rounded"
          />
          <select
            value={categoryFilter}
            onChange={(e) => setCategoryFilter(e.target.value as Category | 'All')}
            className="p-2 border rounded"
          >
            <option value="All">All Categories</option>
            {Object.values(Category).map(category => (
              <option key={category} value={category}>{category}</option>
            ))}
          </select>
          <select
            value={sortOrder}
            onChange={(e) => setSortOrder(e.target.value as 'asc' | 'desc')}
            className="p-2 border rounded"
          >
            <option value="desc">Newest First</option>
            <option value="asc">Oldest First</option>
          </select>
        </div>
      </div>
      <div className="mt-4">
        {weeks.map(weekStart => (
          <div key={weekStart} className="mb-4">
            <button
              onClick={() => toggleWeek(weekStart)}
              className="w-full text-left p-2 bg-gray-100 hover:bg-gray-200 rounded"
            >
              Week of {weekStart} ({groupedNotes[weekStart].length} notes)
              {expandedWeeks.has(weekStart) ? ' ▼' : ' ►'}
            </button>
            {expandedWeeks.has(weekStart) && (
              <ul className="space-y-4 mt-2">
                {groupedNotes[weekStart].map(note => (
                  <li key={note.id} className="bg-white p-4 rounded shadow">
                    <div className="flex justify-between items-start">
                      <div>
                        <Link to={`/edit/${note.id}`} className="text-blue-600 hover:underline text-lg">
                          {note.content.substring(0, 50)}...
                        </Link>
                        <p className="text-sm text-gray-500">Category: {note.category}</p>
                        <p className="text-sm text-gray-500">Created: {new Date(note.created_at).toLocaleString()}</p>
                        <p className="text-sm text-gray-500">Analyzed: {note.analyzed ? 'Yes' : 'No'}</p>
                      </div>
                      <div className="flex flex-col space-y-2">
                        <Link 
                          to={`/edit/${note.id}`} 
                          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm"
                        >
                          Edit
                        </Link>
                        {!note.analyzed && (
                          <button
                            onClick={() => note.id && handleAnalyze(note.id)}
                            className="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-sm flex items-center justify-center"
                            disabled={note.id ? analyzingNotes.has(note.id) : false}
                          >
                            {note.id && analyzingNotes.has(note.id) ? (
                              <Spinner />
                            ) : (
                              'Analyze'
                            )}
                          </button>
                        )}
                        {note.analyzed && (
                          <Link 
                            to={`/analyze/${note.id}`} 
                            className="bg-purple-500 hover:bg-purple-700 text-white font-bold py-1 px-2 rounded text-sm"
                          >
                            View Analysis
                          </Link>
                        )}
                      </div>
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default NotesList;