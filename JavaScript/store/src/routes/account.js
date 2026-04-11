import Router from "express";

const env = process.env.NODE_ENV || "prod";
const router = Router();

const items = [
  { name: "Item 1", description: "description", price: 10 },
  { name: "Item 2", description: "description", price: 15 },
  { name: "Item 3", description: "description", price: 20 },
  { name: "Item 4", description: "description", price: 25 },
  { name: "Item 5", description: "description", price: 30 },
  { name: "Item 6", description: "description", price: 35 },
];

router.get("/", (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: Account view`);
  }

  const { user } = req;
  if (!user) {
    res.redirect("/");
    return;
  }

  res.render("account", { title: "Account", items, user });
});

router.get("/logout", (req, res, next) => {
  if (env === "dev") {
    console.log(`${req.baseUrl || "/"} - ${req.method} :: logged out`);
  }

  res.redirect("/");
});

export default router;
