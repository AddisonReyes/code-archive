import Router from "express";

const env = process.env.NODE_ENV || "prod";
const router = Router();

router.get("/", (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: Home view`);
  }

  res.render("home", null);
});

export default router;
