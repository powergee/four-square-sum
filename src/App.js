import { Typography, IconButton } from '@mui/material';
import { BrowserRouter, Route, Routes } from "react-router-dom";
import GitHubIcon from '@mui/icons-material/GitHub';
import { Calculator } from "./pages";
import './App.css';

function App() {
  return (
    <div className="app-root">
      <div className="app-header">
        <div className="app-header-inner">
          <div className="app-title-layout">
            <Typography variant="h2">Lagrange's four-square calculator</Typography>
          </div>
        </div>
      </div>

      <div className="app-layout">
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<Calculator />} />
          </Routes>
        </BrowserRouter>
      </div>

      <footer>
        <IconButton aria-label="Github">
          <GitHubIcon></GitHubIcon>
        </IconButton>
      </footer>
    </div>
  );
}

export default App;
