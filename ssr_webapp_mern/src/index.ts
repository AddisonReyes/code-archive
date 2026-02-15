import express, { Request, Response } from "express";

const port: number = 3000;

const app = express();

app.get("/", (req: Request, res: Response) => {
  res.send("Home route");
});

app.listen(port, () => {
  console.log(`Server listening on http://localhost:${port}`);
});
