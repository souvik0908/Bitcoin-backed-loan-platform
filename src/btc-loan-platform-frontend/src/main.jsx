import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.jsx';
import './index.css'; // global styles
import { AuthProvider } from './auth/AuthProvider';

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>

    <AuthProvider>
      <App />
    </AuthProvider>
    {/* <App /> */}
  </React.StrictMode>
);
