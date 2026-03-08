import { useState, CreateElement } from "react";

import Students from "./Studdents";
import Message from "./Message";
import Header from "./Header";

const title = "Welcome to React!";

function App() {
  const [notificationCount, setNotificationCount] = useState(0);
  const [isLoggedIn, setIsLoggedIn] = useState(false);
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

      <Students />

      {/* Using a JavaScript variable */}
      <p>Me: {name}</p>

      {/* Using a function  */}
      <p>{formatMessage(name)}</p>

      {/* Inline styles */}
      <h2 style={headingStyle}>Inline Styles</h2>

      {/* Conditional rendering */}
      {isLoggedIn ? (
        <div>
          <p>Welcome back, {name}!</p>
          <button onClick={() => setIsLoggedIn(false)}>log out</button>
        </div>
      ) : (
        <div>
          <p>
            Please <button onClick={() => setIsLoggedIn(true)}>log in</button>{" "}
            to see your messages. ({notificationCount} new messages)
          </p>
          <button onClick={() => setNotificationCount(notificationCount + 1)}>
            Refresh Messages
          </button>
        </div>
      )}

      {isLoggedIn && (
        <div>
          <p>You have {notificationCount} new messages.</p>
          <button onClick={() => setNotificationCount(0)}>Mark as read</button>
        </div>
      )}
    </div>
  );
}

export default App;
