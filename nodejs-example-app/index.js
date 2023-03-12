import { createServer } from "http";
import { collectDefaultMetrics, register } from "prom-client";

collectDefaultMetrics();

const server = createServer(async (req, res) => {
  if (req.url === "/metrics") {
    console.log("Handling /metrics");
    res.write(await register.metrics());
    res.end();
  }
});
server.listen(3000, () => console.log("Server started"))
