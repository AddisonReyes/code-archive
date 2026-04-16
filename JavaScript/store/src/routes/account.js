import emailjs from "@emailjs/nodejs";
import Router from "express";
import bcrypt from "bcrypt";

import Item from "../models/item.js";
import User from "../models/user.js";

const port = process.env.BACKEND_PORT || 3000;
const env = process.env.NODE_ENV || "prod";
const router = Router();

const emailjsPrivateKey = process.env.EMAILJS_PRIVATE_KEY;
const emailjsPublicKey = process.env.EMAILJS_PUBLIC_KEY;
const emailjsServiceId = process.env.EMAILJS_SERVICE_ID;
const emailjsTemplateId = process.env.EMAILJS_TEMPLATE_ID;

function randomString(length) {
  let text = "";
  const possibleChars =
    "QWERTYUIOPLKJHGFDSAZXCVBNMqwertyuioplkjhgfdsazxcvbnm1234567890";
  for (let i = 0; i < length; i++) {
    text += possibleChars.charAt(
      Math.floor(Math.random() * possibleChars.length),
    );
  }

  return text;
}

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

router.post("/reset-password", async (req, res, next) => {
  const { email } = req.body;
  const user = await User.findOne({ email });

  if (user === null) {
    next("This user doesn\'t exists");
    return;
  }

  user.nonce = randomString(8);
  user.passwordResetTime = new Date();

  const link =
    env === "dev"
      ? `http:localhost:${port}/account/password-reset?nonce=${user.nonce}&id=${user._id}`
      : `https:store/account/password-reset?nonce=${user.nonce}&id=${user._id}`;
  const templateParams = { link, email };

  emailjs
    .send(emailjsServiceId, emailjsTemplateId, templateParams, {
      publicKey: emailjsPublicKey,
      privateKey: emailjsPrivateKey,
    })
    .then(
      (response) => {
        if (env === "dev") {
          console.log(
            `${req.baseUrl || "/"} - ${req.method} :: Sending email...`,
          );
          console.log("SUCCESS!", response.status, response.text);
        }
      },
      (err) => {
        if (env === "dev") {
          console.log(
            `${req.baseUrl || "/"} - ${req.method} :: Sending email...`,
          );
          console.log("FAILED...", err);
        }
      },
    );

  await user.save();
});

router.get("/password-reset", async (req, res, next) => {
  const { nonce, id } = req.query;

  if (nonce == null || id == null) {
    next("Invalid Request");
    return;
  }

  const user = await User.findById(id);
  if (user == null) {
    next("Invalid Request");
    return;
  }

  if (user.passwordResetTime == null || user.nonce == null) {
    next("Invalid Request");
    return;
  }

  if (user.nonce != nonce) {
    next("Invalid Request");
    return;
  }

  const now = new Date();
  const diff = now - user.passwordResetTime;
  const seconds = diff / 1000;

  if (seconds > 24 * 60 * 60) {
    next("Invalid Request");
    return;
  }

  const data = { title: "Reset password", id, nonce };
  res.render("password", data);
});

router.post("/new-password", async (req, res, next) => {
  const { password1, password2, nonce, id } = req.body;

  console.log(password1, password2, nonce, id, req.body.id);

  if (password1 == null || password2 == null || nonce == null || id == null) {
    next("Invalid Request");
    return;
  }

  if (password1 != password2) {
    next("Password do not match");
    return;
  }

  const user = await User.findById(id);
  if (user == null) {
    next("Invalid Request");
    return;
  }

  if (user.passwordResetTime == null || user.nonce == null) {
    next("Invalid Request");
    return;
  }

  if (user.nonce != nonce) {
    next("Invalid Request");
    return;
  }

  const password = bcrypt.hashSync(password1, 10);
  user.password = password;
  await user.save();

  if (env === "dev") {
    console.log(
      `${req.baseUrl} - ${req.method} :: User password changed - ${user.email}`,
    );
  }

  res.redirect("/");
});

export default router;
