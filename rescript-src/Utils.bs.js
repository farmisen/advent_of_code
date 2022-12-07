// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Fs from "fs";
import * as Js_array from "rescript/lib/es6/js_array.js";
import * as Caml_option from "rescript/lib/es6/caml_option.js";
import * as Caml_exceptions from "rescript/lib/es6/caml_exceptions.js";

var WentSouth = /* @__PURE__ */Caml_exceptions.create("Utils.WentSouth");

function loadInput(day) {
  var path = "puzzles/day_" + day.toString().padStart(2, "0") + "/input.txt";
  return Fs.readFileSync(path, {
                encoding: "UTF-8"
              }).toString();
}

function sortNumbers(param) {
  return Js_array.sortInPlaceWith((function (a, b) {
                return a - b | 0;
              }), param);
}

function unwrapOrRaise(exp, a) {
  if (a !== undefined) {
    return Caml_option.valFromOption(a);
  }
  throw exp;
}

export {
  WentSouth ,
  loadInput ,
  sortNumbers ,
  unwrapOrRaise ,
}
/* fs Not a pure module */