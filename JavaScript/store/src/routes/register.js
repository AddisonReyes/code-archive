import passport from "passport";
import Router from "express";

const router = Router();

router.post(
  "/",
  passport.authenticate("localRegister", {
    successRedirect: "/account",
  }),
);

export default router;
