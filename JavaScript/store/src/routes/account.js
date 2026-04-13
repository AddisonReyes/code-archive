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

  const interestedItems = await Item.find({ interested: user._id });
  const items = await Item.find({});

  res.render("account", { title: "Account", interestedItems, items, user });
});

router.get("/add-item/:id", async (req, res, next) => {
  const { user } = req;
  if (!user) {
    res.redirect("/");
    return;
  }

  const item = await Item.findById(req.params.id);
  if (item.interested.indexOf(user._id) === -1) {
    item.interested.push(user._id);
    await item.save();
  }
  if (env === "dev") {
    console.log(
      `${req.baseUrl || "/"} - ${req.method} :: Item \'${item.name}\' added `,
    );
  }

  res.redirect("/account");
});

router.get("/remove-item/:id", async (req, res, next) => {
  const { user } = req;
  if (!user) {
    res.redirect("/");
    return;
  }

  const item = await Item.findById(req.params.id);
  const index = item.interested.indexOf(user._id);
  if (index !== -1) {
    item.interested.splice(index, 1);
    await item.save();
  }
  if (env === "dev") {
    console.log(
      `${req.baseUrl || "/"} - ${req.method} :: Item \'${item.name}\' removed `,
    );
  }

  res.redirect("/account");
});

router.get("/logout", (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: logged out`);
  }

  res.redirect("/");
});

export default router;
