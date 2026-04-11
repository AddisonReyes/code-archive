import Router from "express";

import User from "../models/user.js";

const env = process.env.NODE_ENV || "prod";
const router = Router();

router.post("/", async (req, res, next) => {
  try {
    const email = req.body.email;
    const user = await User.findOne({ email });

    if (!user) {
      // res.status(400).send({ error: "This user doesn\'t exists" });
      next("This user doesn\'t exists");
    }

    const password = req.body.password;
    if (user.password === password) {
      if (env === "dev") {
        console.log(
          `${req.baseUrl} - ${req.method} :: User logged in - ${email}`,
        );
      }

      res.status(200).send({ message: "Ok" });
    } else {
      // res.status(400).send({ error: "Incorrect Password" });
      next("Incorrect Password");
    }
  } catch (error) {
    // res.status(400).send(error);
    next(error);
  }
});

export default router;
