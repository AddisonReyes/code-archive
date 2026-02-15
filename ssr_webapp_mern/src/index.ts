import express, { Request, Response } from "express";

import homeRoute from "./routes/home.js";
import registerRoute from "./routes/register.js";

const port: string = "3000";
const app = express();

app.use(express.urlencoded({ extended: false }));
app.use(express.json());

app.set("view engine", "ejs");
app.set("views", "./views");

app.use(express.static("public"));

app.use("/", homeRoute);
app.use("/", registerRoute);

app.listen(port, () => {
  console.log(`Server listening on http://localhost:${port}`);
});
