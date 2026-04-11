import mongoose from "mongoose";
import express from "express";
import "dotenv/config";

import homeRoute from "./routes/home.js";
import registerRoute from "./routes/register.js";
import loginRoute from "./routes/login.js";

const mongo_url = process.env.MONGO_URL;
const port = process.env.BACKEND_PORT || 3000;
const env = process.env.NODE_ENV || "prod";

mongoose
  .connect(mongo_url)
  .then(() => console.log("Database connection success"))
  .catch((err) => console.log("Database connection failed:", err.message));

const app = express();

app.set("views", "views");
app.set("view engine", "ejs");

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.use(express.static("public"));
app.use("/", homeRoute);
app.use("/register", registerRoute);
app.use("/login", loginRoute);

app.use((err, req, res, next) => {
  console.log(err);
  res.render("error", { message: err });
});

app.listen(port, () => {
  if (env === "dev") {
    console.log(` - - Server on \'DEBUG\' mode - - `);
  }
  console.log(`App running on http://localhost:${port}`);
});
