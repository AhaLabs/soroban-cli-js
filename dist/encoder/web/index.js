import { error as lowering0Callee } from '../../../lib/console.js';

const instantiateCore = WebAssembly.instantiate;

const utf8Decoder = new TextDecoder();

function throwUninitialized() {
  throw new TypeError('Wasm uninitialized use `await $init` first');
}

const utf8Encoder = new TextEncoder();

let utf8EncodedLen = 0;
function utf8Encode(s, realloc, memory) {
  if (typeof s !== 'string') throw new TypeError('expected a string');
  if (s.length === 0) {
    utf8EncodedLen = 0;
    return 1;
  }
  let allocLen = 0;
  let ptr = 0;
  let writtenTotal = 0;
  while (s.length > 0) {
    ptr = realloc(ptr, allocLen, 1, allocLen + s.length);
    allocLen += s.length;
    const { read, written } = utf8Encoder.encodeInto(
    s,
    new Uint8Array(memory.buffer, ptr + writtenTotal, allocLen - writtenTotal),
    );
    writtenTotal += written;
    s = s.slice(read);
  }
  if (allocLen > writtenTotal)
  ptr = realloc(ptr, allocLen, 1, writtenTotal);
  utf8EncodedLen = writtenTotal;
  return ptr;
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

const fetchCompile = url => fetch(url).then(WebAssembly.compileStreaming);

const base64Compile = str => WebAssembly.compile(Uint8Array.from(atob(str), b => b.charCodeAt(0)));

let exports0;
let exports1;
let memory0;
let exports2;
let realloc0;
let postReturn0;
export const encoder = {
  createOp(arg0, arg1, arg2, arg3) {
    if (!_initialized) throwUninitialized();
    const val0 = arg0;
    const len0 = val0.byteLength;
    const ptr0 = realloc0(0, 0, 1, len0 * 1);
    const src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
    (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
    const ptr1 = utf8Encode(arg1, realloc0, memory0);
    const len1 = utf8EncodedLen;
    const ptr2 = utf8Encode(arg2, realloc0, memory0);
    const len2 = utf8EncodedLen;
    const ptr3 = utf8Encode(arg3, realloc0, memory0);
    const len3 = utf8EncodedLen;
    const ret = exports1['encoder#create-op'](ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    let variant6;
    switch (dataView(memory0).getUint8(ret + 0, true)) {
      case 0: {
        const ptr4 = dataView(memory0).getInt32(ret + 4, true);
        const len4 = dataView(memory0).getInt32(ret + 8, true);
        const result4 = new Uint8Array(memory0.buffer.slice(ptr4, ptr4 + len4 * 1));
        variant6= {
          tag: 'ok',
          val: result4
        };
        break;
      }
      case 1: {
        const ptr5 = dataView(memory0).getInt32(ret + 4, true);
        const len5 = dataView(memory0).getInt32(ret + 8, true);
        const result5 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr5, len5));
        variant6= {
          tag: 'err',
          val: result5
        };
        break;
      }
      default: {
        throw new TypeError('invalid variant discriminant for expected');
      }
    }
    postReturn0(ret);
    if (variant6.tag === 'err') {
      throw new ComponentError(variant6.val);
    }
    return variant6.val;
  },
  run(arg0, arg1, arg2, arg3) {
    if (!_initialized) throwUninitialized();
    const val0 = arg0;
    const len0 = val0.byteLength;
    const ptr0 = realloc0(0, 0, 1, len0 * 1);
    const src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
    (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
    const ptr1 = utf8Encode(arg1, realloc0, memory0);
    const len1 = utf8EncodedLen;
    const ptr2 = utf8Encode(arg2, realloc0, memory0);
    const len2 = utf8EncodedLen;
    const ptr3 = utf8Encode(arg3, realloc0, memory0);
    const len3 = utf8EncodedLen;
    const ret = exports1['encoder#run'](ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    let variant6;
    switch (dataView(memory0).getUint8(ret + 0, true)) {
      case 0: {
        const ptr4 = dataView(memory0).getInt32(ret + 4, true);
        const len4 = dataView(memory0).getInt32(ret + 8, true);
        const result4 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr4, len4));
        variant6= {
          tag: 'ok',
          val: result4
        };
        break;
      }
      case 1: {
        const ptr5 = dataView(memory0).getInt32(ret + 4, true);
        const len5 = dataView(memory0).getInt32(ret + 8, true);
        const result5 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr5, len5));
        variant6= {
          tag: 'err',
          val: result5
        };
        break;
      }
      default: {
        throw new TypeError('invalid variant discriminant for expected');
      }
    }
    postReturn0(ret);
    if (variant6.tag === 'err') {
      throw new ComponentError(variant6.val);
    }
    return variant6.val;
  },
  decodeRet(arg0, arg1, arg2) {
    if (!_initialized) throwUninitialized();
    const val0 = arg0;
    const len0 = val0.byteLength;
    const ptr0 = realloc0(0, 0, 1, len0 * 1);
    const src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
    (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
    const ptr1 = utf8Encode(arg1, realloc0, memory0);
    const len1 = utf8EncodedLen;
    const val2 = arg2;
    const len2 = val2.byteLength;
    const ptr2 = realloc0(0, 0, 1, len2 * 1);
    const src2 = new Uint8Array(val2.buffer || val2, val2.byteOffset, len2 * 1);
    (new Uint8Array(memory0.buffer, ptr2, len2 * 1)).set(src2);
    const ret = exports1['encoder#decode-ret'](ptr0, len0, ptr1, len1, ptr2, len2);
    let variant5;
    switch (dataView(memory0).getUint8(ret + 0, true)) {
      case 0: {
        const ptr3 = dataView(memory0).getInt32(ret + 4, true);
        const len3 = dataView(memory0).getInt32(ret + 8, true);
        const result3 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr3, len3));
        variant5= {
          tag: 'ok',
          val: result3
        };
        break;
      }
      case 1: {
        const ptr4 = dataView(memory0).getInt32(ret + 4, true);
        const len4 = dataView(memory0).getInt32(ret + 8, true);
        const result4 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr4, len4));
        variant5= {
          tag: 'err',
          val: result4
        };
        break;
      }
      default: {
        throw new TypeError('invalid variant discriminant for expected');
      }
    }
    postReturn0(ret);
    if (variant5.tag === 'err') {
      throw new ComponentError(variant5.val);
    }
    return variant5.val;
  },
  
};

let _initialized = false;
export const $init = (async() => {
  const module0 = fetchCompile(new URL('./index.core.wasm', import.meta.url));
  const module1 = base64Compile('AGFzbQEAAAABBgFgAn9/AAMCAQAEBQFwAQEBBxACATAAAAgkaW1wb3J0cwEACg0BCwAgACABQQARAAALACAEbmFtZQEZAQAWaW5kaXJlY3QtY29uc29sZS1lcnJvcg==');
  const module2 = base64Compile('AGFzbQEAAAABBgFgAn9/AAIVAgABMAAAAAgkaW1wb3J0cwFwAQEBCQcBAEEACwEA');
  Promise.all([module0, module1, module2]).catch(() => {});
  ({ exports: exports0 } = await instantiateCore(await module1));
  ({ exports: exports1 } = await instantiateCore(await module0, {
    console: {
      error: exports0['0'],
    },
  }));
  memory0 = exports1.memory;
  
  function lowering0(arg0, arg1) {
    const ptr0 = arg0;
    const len0 = arg1;
    const result0 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr0, len0));
    lowering0Callee(result0);
  }
  ({ exports: exports2 } = await instantiateCore(await module2, {
    '': {
      $imports: exports0.$imports,
      '0': lowering0,
    },
  }));
  realloc0 = exports1.cabi_realloc;
  postReturn0 = exports1['cabi_post_encoder#create-op'];
  _initialized = true;
})();
