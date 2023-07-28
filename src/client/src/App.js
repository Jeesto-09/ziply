import React, { useState } from 'react';
import './App.css';

const App = () => {
  const [longUrl, setLongUrl] = useState('');
  const [shortUrl, setShortUrl] = useState('');
  const [short_name, setShortUrlName] = useState('');

  const handleShortenClick = () => {
    const apiUrl = 'http://zip.ly:8080/api/shorten_url';

    // Create a new FormData object and append the 'orig_url' and 'custom_name' fields
    const formData = new URLSearchParams();
    formData.append('orig_url', longUrl);

    // Only append the 'short_name' field if it has a value
    if (short_name) {
      formData.append('short_name', short_name);
    }

    // Sending the POST request using fetch()
    fetch(apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      // Convert FormData to a URL-encoded string
      body: formData.toString(),
    })
      .then((response) => response.json())
      .then((data) => {
        setShortUrl(data.short_url);
        console.log(data.shortUrl);
      })
      .catch((error) => {
        console.error('Error:', error);
        console.log('Response:', error.response);
      });
  };

  return (
    <div className="App">
      <h1>
        Zip your links <span>shorter</span>
      </h1>
      <div className="url-input-container">
        <input
          type="text"
          value={longUrl}
          onChange={(e) => setLongUrl(e.target.value)}
          placeholder="Paste your long URL"
        />
        <input
          type="text"
          value={short_name}
          onChange={(e) => setShortUrlName(e.target.value)}
          placeholder="Customize the back half of your link (optional)"
        />
        <button onClick={handleShortenClick}>shorten</button>
      </div>
      {shortUrl && (
        <p className="short-url">
          Shortened URL: <a href={shortUrl} target="_blank" rel="noopener noreferrer">{shortUrl}</a>
        </p>
      )}
    </div>
  );
};

export default App;
