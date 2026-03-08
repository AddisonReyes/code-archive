import { useState } from "react";

function Login({ name, loggedIn }) {
  const [notificationCount, setNotificationCount] = useState(0);
  const [isLoggedIn, setIsLoggedIn] = useState(loggedIn);

  return (
    <div>
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
          <button
            onClick={() => {
              setNotificationCount(
                (prevNotificationCount) => prevNotificationCount + 1,
              );
              setNotificationCount(
                (prevNotificationCount) => prevNotificationCount + 1,
              );
              console.log("Notification count updated:", notificationCount);
            }}
          >
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

export default Login;
