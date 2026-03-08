import { CreateElement } from "react";

import Message from "./Message";
import Header from "./Header";

const title = "Welcome to React!";

function App() {
  const name = "Addison";

  const formatMessage = (name) => {
    return `Hello, ${name}!`;
  };

  const headingStyle = {
    color: "teal",
    fontSize: "28px",
    textAlign: "center",
    margin: "20px 0",
  };

  return (
    <div>
      <Header />
      <h1>{title}</h1>
      <Message text="This is a message component." />
      <Message text="This is another message component." />

      {/* Using a JavaScript variable */}
      <p>Me: {name}</p>

      {/* Using a function  */}
      <p>{formatMessage(name)}</p>

      {/* Inline styles */}
      <h2 style={headingStyle}>Inline Styles</h2>
    </div>
  );
}

export default App;
