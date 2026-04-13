import Router from "express";

import Item from "../models/item.js";

const env = process.env.NODE_ENV || "prod";
const router = Router();

router.get("/", async (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: Admin view`);
  }

  const { user } = req;
  if (!user) {
    res.redirect("/");
    return;
  }

  if (!user.isAdmin) {
    res.redirect("/");
    return;
  }

  const items = await Item.find({});

  const data = {
    title: "Admin panel",
    items,
    user,
  };

  res.render("admin", data);
});

router.post("/add-item", async (req, res, next) => {
  const { user } = req;
  if (!user) {
    res.redirect("/");
    return;
  }

  if (!user.isAdmin) {
    res.redirect("/");
    return;
  }

  const newItem = new Item(req.body);
  await newItem.save();

  if (env === "dev") {
    console.log(
      `${req.baseUrl} - ${req.method} :: New item created - ${newItem.name}`,
    );
  }

  res.redirect("/admin");
});

export default router;
