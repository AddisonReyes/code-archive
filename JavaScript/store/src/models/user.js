import mongoose from "mongoose";

const UserSchema = new mongoose.Schema({
  email: {
    type: String,
    default: "",
  },
  password: {
    type: String,
    default: "",
  },
  isAdmin: {
    type: Boolean,
    default: false,
  },
  timestamp: {
    type: Date,
    default: Date.now,
  },
});

const User = mongoose.model("User", UserSchema);

export default User;
