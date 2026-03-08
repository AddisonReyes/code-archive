function ProfileCard({ name, role, isOnline }) {
  return (
    <div
      style={{
        border: "1px solid #ccc",
        padding: "15px",
        marginTop: "20px",
        borderRadius: "8px",
      }}
    >
      <h2>{name}</h2>
      <p>{role}</p>
      <p style={{ color: isOnline ? "green" : "red" }}>
        {isOnline ? "Online" : "Offline"}
      </p>
    </div>
  );
}

export default ProfileCard;
