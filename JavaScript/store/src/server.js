import express from "express";
import homeRoute from "./routes/home.js";

const app = express();

app.use("/", homeRoute);

app.listen(3000, () => {
  console.log("App running on http://localhost:3000");
});
