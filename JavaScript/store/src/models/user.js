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
  nonce: {
    type: String,
    default: null,
  },
  passwordResetTime: {
    default: null,
    type: Date,
  },
});

const User = mongoose.model("User", UserSchema);

export default User;
