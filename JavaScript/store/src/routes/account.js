import Router from "express";

const env = process.env.NODE_ENV || "prod";
const router = Router();

router.get("/", (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: Account view`);
  }

  res.send({
    user: req.user || "Not logged in",
  });
});

router.get("/logout", (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: logged out`);
  }

  req.logout(() => {
    res.send({
      message: "User logged out",
    });
  });
});

export default router;
