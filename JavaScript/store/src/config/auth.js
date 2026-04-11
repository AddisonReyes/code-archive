import LocalStrategy from "passport-local";
import User from "../models/user.js";

const env = process.env.NODE_ENV || "prod";

function auth(passport) {
  passport.serializeUser((user, next) => {
    next(null, user?._id?.toString?.() ?? user?.id);
  });

  passport.deserializeUser(async (id, next) => {
    try {
      if (!id) {
        next(null, false);
        return;
      }

      const user = await User.findById(id);
      next(null, user || false);
    } catch (err) {
      next(err);
    }
  });

  const localLogin = new LocalStrategy(
    {
      usernameField: "email",
      passwordField: "password",
      passReqToCallback: true,
    },
    async (req, email, password, next) => {
      const user = await User.findOne({ email });

      if (user === null) {
        next("This user doesn\'t exists");
        return;
      }

      if (user.password !== password) {
        next("Incorrect Password");
        return;
      }

      if (env === "dev") {
        console.log(
          `${req.baseUrl} - ${req.method} :: User logged in - ${email}`,
        );
      }

      next(null, user);
    },
  );

  const localRegister = new LocalStrategy(
    {
      usernameField: "email",
      passwordField: "password",
      passReqToCallback: true,
    },
    async (req, email, password, next) => {
      const user = await User.findOne({ email });

      if (user !== null) {
        next("This user already exists, please log in.");
        return;
      }

      const newUser = new User({
        email: req.body.email,
        password: req.body.password,
      });

      await newUser.save();

      if (env === "dev") {
        console.log(
          `${req.baseUrl} - ${req.method} :: User registered - ${email}`,
        );
      }

      next(null, newUser);
    },
  );

  passport.use("localRegister", localRegister);
  passport.use("localLogin", localLogin);
}

export default auth;
