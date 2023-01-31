import * as encoder from "./xdr/encoder";
import  * as fs from "node:fs/promises";

let [wasm, fn, args]  = process.argv.slice(2);

console.log(encoder.exports.run(await fs.readFile(wasm), "aaa", fn, args));