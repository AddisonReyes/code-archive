import emailjs from "@emailjs/nodejs";
import Router from "express";

import Item from "../models/item.js";
import User from "../models/user.js";

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

  const templateParams = { email };
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

export default router;
