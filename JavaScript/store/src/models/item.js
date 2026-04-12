import mongoose from "mongoose";

const ItemSchema = new mongoose.Schema({
  name: {
    type: String,
    default: "",
  },
  description: {
    type: String,
    default: "",
  },
  price: {
    type: Number,
    default: 0,
  },
  interested: {
    type: Array,
    default: [],
  },
});

const Item = mongoose.model("User", ItemSchema);

export default Item;
