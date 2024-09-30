const API_BASE_URL = 'http://localhost:8080';

export const api = {
  get: (endpoint: string) => 
    fetch(`${API_BASE_URL}${endpoint}`).then(response => response.json()),
  
  post: (endpoint: string, data: any) => 
    fetch(`${API_BASE_URL}${endpoint}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    }).then(response => response.json()),
  
  put: (endpoint: string, data: any) => 
    fetch(`${API_BASE_URL}${endpoint}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    }).then(response => response.json()),
  
  delete: (endpoint: string) => 
    fetch(`${API_BASE_URL}${endpoint}`, {
      method: 'DELETE',
    }).then(response => response.json()),
};
