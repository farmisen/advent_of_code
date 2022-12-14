// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Js_array from "rescript/lib/es6/js_array.js";
import * as Js_string from "rescript/lib/es6/js_string.js";
import * as Belt_SetString from "rescript/lib/es6/belt_SetString.js";

function findMarkerOfSize(size, packets) {
  return Js_array.findIndexi((function (param, idx) {
                return Belt_SetString.size(Belt_SetString.fromArray(Js_string.split("", Js_string.slice(idx, idx + size | 0, packets)))) === size;
              }), Js_string.split("", packets)) + size | 0;
}

function part01(input) {
  return findMarkerOfSize(4, input);
}

function part02(input) {
  return findMarkerOfSize(14, input);
}

export {
  findMarkerOfSize ,
  part01 ,
  part02 ,
}
/* No side effect */
