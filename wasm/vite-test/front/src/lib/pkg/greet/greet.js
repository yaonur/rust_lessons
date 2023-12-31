import * as wasm from "./greet_bg.wasm";
import { __wbg_set_wasm } from "./greet_bg.js";
__wbg_set_wasm(wasm);
export * from "./greet_bg.js";
