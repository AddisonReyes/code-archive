import express from "express";

import homeRoute from "./routes/home.js";
import registerRoute from "./routes/register.js";

const app = express();

app.set("views", "views");
app.set("view engine", "ejs");

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.use(express.static("public"));
app.use("/", homeRoute);
app.use("/register", registerRoute);

app.listen(3000, () => {
  console.log("App running on http://localhost:3000");
});
