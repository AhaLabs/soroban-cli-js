import * as fs from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { mkdirSync  } from "node:fs";

function build_component(name, profile) {
    mkdirSync(`dist/${name}`, { recursive: true });
    console.log(name)
    const create = `jsct new target/wasm32-unknown-unknown/${profile}/${name}.wasm -o dist/${name}/index.wasm`;
    const transpile = `jsct transpile dist/${name}/index.wasm --map console=../../../lib/console.js --out-dir dist/${name}/node`;
    const transpile2 = `jsct transpile dist/${name}/index.wasm --map console=../../../lib/console.js --out-dir dist/${name}/web --tla-compat --no-nodejs-compat`;
    let res = spawnSync("yarn", create.split(" "));
    
    if (res.stdout) {
        console.log(res.stdout.toString());
    }
    res = spawnSync("yarn", transpile.split(" "));
    
    if (res.stdout) {
        console.log(res.stdout.toString());
    }
    res = spawnSync("yarn", transpile2.split(" "));
   
    if (res.stdout) {
        console.log(res.stdout.toString());
    }
}

async function build(profile) {
    console.log(`Building ${profile}...`);
    const dir = (await fs.readdir(`target/wasm32-unknown-unknown/${profile}`)).filter(file => file.endsWith(".wasm"));
    console.log(dir)
    dir.forEach(file => build_component(file.replace(".wasm", ""), profile));
}

await build(process.argv[2]);