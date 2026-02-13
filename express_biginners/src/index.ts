import express, { Response, Request, NextFunction } from "express";

const app = express();

app.get("/", (req: Request, res: Response, next: NextFunction) => {
  res.send("Express fundamentals");
});

app.get("/json", (req: Request, res: Response, next: NextFunction) => {
  const data = {
    name: "Addison",
    location: "Dominican Republic",
  };
  res.json(data);
});

app.get("/html", (req: Request, res: Response, next: NextFunction) => {
  const html = "<html><h1>This is a HTML response</h1></html>";
  res.send(html);
});

app.get("/query", (req: Request, res: Response, next: NextFunction) => {
  const query = req.query;
  res.json(query);
});

app.get(
  "/params/:name/:location/:occupation",
  (req: Request, res: Response, next: NextFunction) => {
    const params = req.params;
    res.json({ params });
  },
);

app.listen("3000", () => {
  console.log("Server listening on http://localhost:3000");
});
