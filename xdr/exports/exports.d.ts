export namespace Exports {
  export function createOp(wasm: Uint8Array | ArrayBuffer, contractId: string, funcName: string, jsonArgs: string): Uint8Array;
  export function decodeRet(wasm: Uint8Array | ArrayBuffer, funcName: string, xdrArgs: Uint8Array | ArrayBuffer): string;
  export function run(wasm: Uint8Array | ArrayBuffer, contractId: string, funcName: string, jsonArgs: string): string;
}
