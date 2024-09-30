import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { api } from '../utils/api';
import { Note } from '../types/Note';

const NoteAnalyzer: React.FC = () => {
  const [note, setNote] = useState<Note | null>(null);
  const { id } = useParams<{ id: string }>();

  useEffect(() => {
    if (id) {
      api.get(`/notes/${id}`)
        .then(data => setNote(data))
        .catch(error => console.error('Error fetching note:', error));
    }
  }, [id]);

  if (!note) {
    return <div className="flex justify-center items-center h-64">
      <div className="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-gray-900"></div>
    </div>;
  }

  return (
    <div className="max-w-2xl mx-auto mt-8 p-6 bg-white rounded-lg shadow-lg">
      <h2 className="text-3xl font-bold mb-4">Note Analysis</h2>
      <div className="mb-6">
        <h3 className="text-xl font-semibold mb-2">Content:</h3>
        <div className="bg-gray-100 p-4 rounded-md">
          <ReactMarkdown remarkPlugins={[remarkGfm]}>{note.content}</ReactMarkdown>
        </div>
      </div>
      <div className="mb-6">
        <h3 className="text-xl font-semibold mb-2">Category:</h3>
        <p className="bg-blue-100 text-blue-800 px-3 py-1 rounded-full inline-block">{note.category}</p>
      </div>
      <div className="mb-6">
        <h3 className="text-xl font-semibold mb-2">Analysis:</h3>
        {note.analyzed ? (
          <div className="bg-green-100 p-4 rounded-md">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{note.analysis || 'No analysis available.'}</ReactMarkdown>
          </div>
        ) : (
          <p className="text-yellow-600">This note has not been analyzed yet.</p>
        )}
      </div>
      <div className="text-sm text-gray-600">
        <p>Created: {new Date(note.created_at).toLocaleString()}</p>
        <p>Last updated: {new Date(note.updated_at).toLocaleString()}</p>
      </div>
    </div>
  );
};

export default NoteAnalyzer;