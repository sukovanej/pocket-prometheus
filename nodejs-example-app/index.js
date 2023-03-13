import { createServer } from "http";
import { Counter, collectDefaultMetrics, register } from "prom-client";

collectDefaultMetrics();

const counter = new Counter({
  name: 'patrik', help: 'patrik'
});

const server = createServer(async (req, res) => {
  if (req.url === "/metrics") {
    console.log("Handling /metrics");
    counter.inc();
    res.write(await register.metrics());
    res.end();
  }
});
server.listen(3000, () => console.log("Server started"))
