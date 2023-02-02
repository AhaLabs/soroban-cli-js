export namespace Invoker {
  export function invoke(wasm: Uint8Array | ArrayBuffer, contractId: string, funcName: string, jsonArgs: string): string;
}
