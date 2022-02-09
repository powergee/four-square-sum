import './App.css';
import { useState } from 'react';
import { findSolution } from './FourSquare';

function App() {
  const [number, setNumber] = useState("0");
  const [result, setResult] = useState(["0", "0", "0", "0"]);

  function handleChange(e) {
    setNumber(e.target.value);
    setResult(findSolution(e.target.value))
  }

  return (
    <div className='app-root'>
      <input type='number' onChange={handleChange}></input>
      <div className='app-results'>
        <span>{`${number}=${result.map(r => `${r}²`).join('+')}`}</span>
      </div>
    </div>
  );
}

export default App;
