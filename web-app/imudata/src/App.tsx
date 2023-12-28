import React from 'react';
import {
  BrowserRouter as Router,
  Routes,
  Route,
} from 'react-router-dom';
import './App.css';
import Home from "./pages/Home";
import NavBar from "./components/NavBar";
import About from "./pages/About";
import Plots from "./pages/Plots";
import Upload from "./pages/Upload";

const App = () =>{
  return (
    <Router>
      <NavBar/>
      <Routes>
        <Route path='/' element={<Home/>}/>
        <Route path='/about' element={<About/>}/>
        <Route path='/upload' element={<Upload/>}/>
        <Route path='/plots' element={<Plots/>}/>
      </Routes>
    </Router>
  );
}

export default App;
