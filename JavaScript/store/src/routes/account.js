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

router.get("/add-item/:id", async (req, res, next) => {
  const { user } = req;
  if (!user) {
    res.redirect("/");
    return;
  }

  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: Adding item...`);
  }

  const item = await Item.findById(req.params.id);
  if (item.interested.indexOf(user._id) === -1) {
    item.interested.push(user._id);
    await item.save();
  }
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: Item added `);
  }

  res.json({
    item,
    user,
  });
});

router.get("/logout", (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: logged out`);
  }

  res.redirect("/");
});

export default router;
