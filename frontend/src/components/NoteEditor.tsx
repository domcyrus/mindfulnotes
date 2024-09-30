import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
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
  const [isPreviewMode, setIsPreviewMode] = useState(false);
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
    <div className="max-w-4xl mx-auto mt-8">
      <h2 className="text-3xl font-bold mb-6">{id ? 'Edit Note' : 'Create New Note'}</h2>
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="flex space-x-4">
          <div className="w-1/2">
            <label htmlFor="category" className="block text-sm font-medium text-gray-700 mb-1">Category</label>
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
          <div className="w-1/2 flex items-end">
            <button
              type="button"
              onClick={() => setIsPreviewMode(!isPreviewMode)}
              className="bg-gray-200 hover:bg-gray-300 text-gray-800 font-bold py-2 px-4 rounded"
            >
              {isPreviewMode ? 'Edit Mode' : 'Preview Mode'}
            </button>
          </div>
        </div>
        
        {isPreviewMode ? (
          <div className="border rounded-md p-4 prose max-w-none">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{note.content}</ReactMarkdown>
          </div>
        ) : (
          <div>
            <label htmlFor="content" className="block text-sm font-medium text-gray-700 mb-1">Content</label>
            <textarea
              id="content"
              value={note.content}
              onChange={e => setNote({ ...note, content: e.target.value })}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
              rows={15}
              required
            />
          </div>
        )}
        
        {note.analyzed && (
          <div>
            <h3 className="text-xl font-semibold mb-2">Analysis</h3>
            <div className="bg-gray-100 p-4 rounded-md prose max-w-none">
              <ReactMarkdown remarkPlugins={[remarkGfm]}>{note.analysis || 'No analysis available.'}</ReactMarkdown>
            </div>
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
    </div>
  );
};

export default NoteEditor;