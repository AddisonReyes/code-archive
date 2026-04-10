import Router from "express";

import User from "../models/user.js";

const env = process.env.NODE_ENV || "prod";
const router = Router();

router.post("/", async (req, res, next) => {
  try {
    const newUser = new User({
      email: req.body.email,
      password: req.body.password,
    });

    await newUser.save();
    if (env === "dev") {
      console.log(
        `${req.baseUrl} - ${req.method} :: User created - User(${newUser})`,
      );
    }

    res.status(200).send(newUser);
  } catch (error) {
    res.status(400).send(error);
  }
});

export default router;
