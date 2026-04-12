import Router from "express";

const env = process.env.NODE_ENV || "prod";
const router = Router();

router.get("/", (req, res, next) => {
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

  const data = {
    title: "Admin",
    user,
  };

  res.render("admin", data);
});

export default router;
