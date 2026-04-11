import expressLayouts from "express-ejs-layouts";
import session from "express-session";
import passport from "passport";
import mongoose from "mongoose";
import express from "express";
import "dotenv/config";

import auth from "./config/auth.js";
import homeRoute from "./routes/home.js";
import accountRoute from "./routes/account.js";
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

auth(passport);
app.use(
  session({
    secret: "kjhqwevbnaet",
    saveUninitialized: true,
    resave: true,
  }),
);

app.use(passport.initialize());
app.use(passport.session());
app.use((req, res, next) => {
  res.locals.user = req.user;
  next();
});

app.use(expressLayouts);
app.set("views", "views");
app.set("view engine", "ejs");
app.set("layout", "layout");

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(express.static("public"));

app.use("/", homeRoute);
app.use("/register", registerRoute);
app.use("/login", loginRoute);
app.use("/account", accountRoute);

app.use((err, req, res, next) => {
  const message =
    err instanceof Error
      ? err.message
      : typeof err === "string"
        ? err
        : "Unexpected error";

  if (env === "dev") {
    console.log(`Error :: ${message}`);
  }

  res.status(500).render("error", { title: "Error", message });
});

app.listen(port, () => {
  if (env === "dev") {
    console.log(` - - Server on \'DEBUG\' mode - - `);
  }
  console.log(`App running on http://localhost:${port}`);
});
