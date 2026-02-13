import express, { Response, Request, NextFunction } from "express";

const app = express();

app.get("/", (req: Request, res: Response, next: NextFunction) => {
  res.send("This is a response");
});

app.listen("3000", () => {
  console.log("Server listening on http://localhost:3000");
});
