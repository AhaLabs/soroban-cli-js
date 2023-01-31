import {exports} from "./xdr/encoder.js";
import  * as fs from "node:fs/promises";


let [wasm, contract_id, fn, args]  = process.argv.slice(2);

console.log(exports.run(await fs.readFile(wasm), contract_id, fn, args));