import React, { useState } from 'react';
import { api } from '../utils/api';

const AIGenerator: React.FC = () => {
  const [prompt, setPrompt] = useState('');
  const [generatedText, setGeneratedText] = useState('');

  const handleGenerate = async () => {
    try {
      const data = await api.post('/generate', { prompt });
      setGeneratedText(data.generated_text);
    } catch (error) {
      console.error('Error generating text:', error);
    }
  };

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold">AI Text Generator</h2>
      <div>
        <label htmlFor="prompt" className="block text-sm font-medium text-gray-700">Prompt</label>
        <input
          type="text"
          id="prompt"
          value={prompt}
          onChange={e => setPrompt(e.target.value)}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        />
      </div>
      <button onClick={handleGenerate} className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">
        Generate Text
      </button>
      {generatedText && (
        <div>
          <h3 className="text-xl font-semibold">Generated Text:</h3>
          <p>{generatedText}</p>
        </div>
      )}
    </div>
  );
};

export default AIGenerator;