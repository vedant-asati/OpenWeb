import logo from './logo.svg';
import './App.css';
import Login from './components/login';
import Logout from './components/logout';
// require('dotenv').config();
// import dotenv from 'dotenv';
import { useEffect } from 'react';
import { gapi } from 'gapi-script';

const clientId = '343890256332-otlmb1slng5u0plp32sp2kt2j0tivh74.apps.googleusercontent.com';

function App() {
  useEffect(() => {
    function start() {
      gapi.client.init({
        clientId: clientId,
        scope: ""
      })
    };
    gapi.load('client:auth2',start);
  })

  return (
    <div className="App">
      <Login />
      <Logout />
    </div>
  );
}

export default App;
