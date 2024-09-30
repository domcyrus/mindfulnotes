import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { api } from '../utils/api';
import { Note, Category } from '../types/Note';

const NoteEditor: React.FC = () => {
  const [note, setNote] = useState<Note>({
    content: '',
    analyzed: false,
    category: Category.Unspecified,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  });
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  useEffect(() => {
    if (id) {
      api.get(`/notes/${id}`)
        .then(data => setNote(data))
        .catch(error => console.error('Error fetching note:', error));
    }
  }, [id]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (id) {
        await api.put(`/notes/${id}`, note);
      } else {
        await api.post('/notes', note);
      }
      navigate('/');
    } catch (error) {
      console.error('Error saving note:', error);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4 max-w-2xl mx-auto mt-8">
      <div>
        <label htmlFor="content" className="block text-sm font-medium text-gray-700">Content</label>
        <textarea
          id="content"
          value={note.content}
          onChange={e => setNote({ ...note, content: e.target.value })}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
          rows={10}
          required
        />
      </div>
      <div>
        <label htmlFor="category" className="block text-sm font-medium text-gray-700">Category</label>
        <select
          id="category"
          value={note.category}
          onChange={e => setNote({ ...note, category: e.target.value as Category })}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        >
          {Object.values(Category).map(category => (
            <option key={category} value={category}>{category}</option>
          ))}
        </select>
      </div>
      {note.analyzed && (
        <div>
          <label htmlFor="analysis" className="block text-sm font-medium text-gray-700">Analysis</label>
          <textarea
            id="analysis"
            value={note.analysis || ''}
            readOnly
            className="mt-1 block w-full rounded-md border-gray-300 bg-gray-100 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
            rows={5}
          />
        </div>
      )}
      <div className="flex justify-between items-center">
        <button type="submit" className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
          Save Note
        </button>
        {id && !note.analyzed && (
          <button 
            type="button" 
            onClick={() => navigate(`/analyze/${id}`)} 
            className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
          >
            Analyze Note
          </button>
        )}
      </div>
    </form>
  );
};

export default NoteEditor;