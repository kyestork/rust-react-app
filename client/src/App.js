import React from "react";
import './App.css';

function App() {
  const [data, setData] = React.useState(null);

  React.useEffect(() => {
    fetch("/api")
      .then((res) => res.json())
      .then((data) => setData(data.data.text));
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src="./derpibooru_2104764.png" className="App-logo" alt="logo" />
        <p>
          Data from the backend: {!data ? "Loading..." : data}
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
