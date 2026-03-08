import { useState } from "react";

import ProfileCard from "./ProfileCard";
import Students from "./Students";
import Message from "./Message";
import Header from "./Header";
import Login from "./login";
import State from "./State";

const title = "Welcome to React!";

function App() {
  const [isOnline, setIsOnline] = useState(true);
  const [name, setName] = useState("");

  const handleInputChange = (event) => {
    setName(event.target.value);
  };

  return (
    <div>
      <h1>{title}</h1>
      <div>
        <input
          type="text"
          placeholder="Enter your name"
          value={name}
          onChange={handleInputChange}
        />
        {name && <p>Hello, {name}!</p>}
      </div>

      {name && <Login name={name} loggedIn={false} />}

      <br />
      <hr />
      <br />

      <p>Is Online: {isOnline ? "Yes" : "No"}</p>
      <button onClick={() => setIsOnline((prevStatus) => !prevStatus)}>
        Toggle Online Status
      </button>
      <ProfileCard
        name="Addison"
        role="Software Engineer"
        isOnline={isOnline}
      />
      <ProfileCard name="Daniel" role="React Developer" isOnline={isOnline} />

      {/* <Header /> */}
      {/* <Message text="This is a message component." /> */}
      {/* <Message text="This is another message component." /> */}
      {/* <Students /> */}
      {/* <State /> */}
    </div>
  );
}

export default App;
