// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Fs from "fs";
import * as Os from "os";
import * as Curry from "rescript/lib/es6/curry.js";
import * as Js_array from "rescript/lib/es6/js_array.js";
import * as Js_string from "rescript/lib/es6/js_string.js";
import * as Belt_Range from "rescript/lib/es6/belt_Range.js";
import * as Caml_array from "rescript/lib/es6/caml_array.js";
import * as Caml_int32 from "rescript/lib/es6/caml_int32.js";
import * as Belt_Option from "rescript/lib/es6/belt_Option.js";
import * as Caml_option from "rescript/lib/es6/caml_option.js";
import * as Caml_exceptions from "rescript/lib/es6/caml_exceptions.js";

var WentSouth = /* @__PURE__ */Caml_exceptions.create("Utils.WentSouth");

function loadInput(day) {
  var path = "../puzzles/day_" + day.toString().padStart(2, "0") + "/input.txt";
  return Fs.readFileSync(path, {
                encoding: "UTF-8"
              }).toString();
}

function sortNumbers(param) {
  return Js_array.sortInPlaceWith((function (a, b) {
                return a - b | 0;
              }), param);
}

function mapRange(from, to, apply) {
  if (to < from) {
    return [];
  }
  var ary = [];
  Belt_Range.forEach(from, to, (function (idx) {
          Js_array.push(Curry._1(apply, idx), ary);
        }));
  return ary;
}

function abs(a) {
  var match = a < 0;
  if (match) {
    return -a | 0;
  } else {
    return a;
  }
}

function unwrapOrRaise(exp, a) {
  if (a !== undefined) {
    return Caml_option.valFromOption(a);
  }
  throw exp;
}

function first(arr) {
  var match = arr.length;
  if (match !== 0) {
    return Caml_array.get(arr, 0);
  }
  throw {
        RE_EXN_ID: WentSouth,
        Error: new Error()
      };
}

function last(arr) {
  var match = arr.length;
  if (match !== 0) {
    return Caml_array.get(arr, arr.length - 1 | 0);
  }
  throw {
        RE_EXN_ID: WentSouth,
        Error: new Error()
      };
}

function loadLines(day) {
  return Js_string.split(Os.EOL, loadInput(day));
}

function pushTo(arr, item) {
  Js_array.push(item, arr);
  return arr;
}

function unshiftArray(arr, item) {
  Js_array.unshift(item, arr);
  return arr;
}

function reverseArray(arr) {
  arr.reverse();
  return arr;
}

function sortArrayWith($$with, arr) {
  Js_array.sortInPlaceWith($$with, arr);
  return arr;
}

function sliceArrayAt(idx, arr) {
  return [
          Js_array.slice(0, idx, arr),
          Js_array.sliceFrom(idx, arr)
        ];
}

function toChunks(size, arr) {
  return Js_array.reducei((function (accumulator, item, index) {
                var chunk;
                if (Caml_int32.mod_(index, size) === 0) {
                  chunk = [];
                } else {
                  var __x = accumulator.pop();
                  chunk = Belt_Option.getWithDefault(__x === undefined ? undefined : Caml_option.some(__x), []);
                }
                return pushTo(accumulator, pushTo(chunk, item));
              }), [], arr);
}

function arrayOfSize(size, valueAt) {
  var arr = [];
  for(var idx = 0; idx < size; ++idx){
    Js_array.push(Curry._1(valueAt, idx), arr);
  }
  return arr;
}

export {
  WentSouth ,
  loadInput ,
  sortNumbers ,
  mapRange ,
  abs ,
  unwrapOrRaise ,
  first ,
  last ,
  loadLines ,
  pushTo ,
  unshiftArray ,
  reverseArray ,
  sortArrayWith ,
  sliceArrayAt ,
  toChunks ,
  arrayOfSize ,
}
/* fs Not a pure module */
