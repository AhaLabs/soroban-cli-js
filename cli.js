import { invoker } from "./dist/invoker/node/index.js";
import  * as fs from "node:fs/promises";

let [wasm, contract_id, fn, args]  = process.argv.slice(2);

console.log(invoker.invoke(await fs.readFile(wasm), contract_id, fn, args));