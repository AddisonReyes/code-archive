import Router from "express";

import Item from "../models/item.js";

const env = process.env.NODE_ENV || "prod";
const router = Router();

router.get("/", async (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: Account view`);
  }

  const { user } = req;
  if (!user) {
    res.redirect("/");
    return;
  }

  const items = await Item.find({});
  res.render("account", { title: "Account", items, user });
});

router.get("/logout", (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: logged out`);
  }

  res.redirect("/");
});

export default router;
