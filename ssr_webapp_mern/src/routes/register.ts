import { Router, Request, Response } from "express";

const endpoint = "/register";
const router = Router();

router.post(endpoint, (req: Request, res: Response) => {
  res.json({
    data: req.body,
  });
});

export default router;
