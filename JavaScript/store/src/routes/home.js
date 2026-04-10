import Router from "express";

const router = Router();

router.get("/", (req, res, next) => {
  res.send("This is from home router!");
});

export default router;
