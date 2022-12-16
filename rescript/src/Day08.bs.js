// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Os from "os";
import * as Utils from "./Utils.bs.js";
import * as Belt_Int from "rescript/lib/es6/belt_Int.js";
import * as Js_array from "rescript/lib/es6/js_array.js";
import * as Js_string from "rescript/lib/es6/js_string.js";
import * as Caml_array from "rescript/lib/es6/caml_array.js";
import * as Caml_int32 from "rescript/lib/es6/caml_int32.js";
import * as Belt_Option from "rescript/lib/es6/belt_Option.js";
import * as Caml_splice_call from "rescript/lib/es6/caml_splice_call.js";

function fromRows(rows) {
  return {
          width: Caml_array.get(rows, 0).length,
          height: rows.length,
          elements: Js_array.concatMany(rows, [])
        };
}

function $$new(width, height, val) {
  return fromRows(Utils.mapRange(0, height - 1 | 0, (function (param) {
                    return Utils.mapRange(0, width - 1 | 0, (function (param) {
                                  return val;
                                }));
                  })));
}

function get(x, y, ary2d) {
  return Caml_array.get(ary2d.elements, x + Math.imul(y, ary2d.width) | 0);
}

function set(x, y, val, ary2d) {
  Caml_array.set(ary2d.elements, x + Math.imul(y, ary2d.width) | 0, val);
  return ary2d;
}

function enumerate(ary2d) {
  return Js_array.mapi((function (val, idx) {
                return {
                        x: Caml_int32.mod_(idx, ary2d.width),
                        y: Caml_int32.div(idx, ary2d.height),
                        val: val
                      };
              }), ary2d.elements);
}

function leftValues(param, ary2d) {
  var x = param.x;
  if (x <= 0) {
    return [];
  }
  var y = param.y;
  return Utils.mapRange(0, x - 1 | 0, (function (n) {
                return get(n, y, ary2d);
              }));
}

function rightValues(param, ary2d) {
  var x = param.x;
  if (x >= (ary2d.width - 1 | 0)) {
    return [];
  }
  var y = param.y;
  return Utils.mapRange(x + 1 | 0, ary2d.width - 1 | 0, (function (n) {
                return get(n, y, ary2d);
              }));
}

function topValues(param, ary2d) {
  var y = param.y;
  if (y <= 0) {
    return [];
  }
  var x = param.x;
  return Utils.mapRange(0, y - 1 | 0, (function (n) {
                return get(x, n, ary2d);
              }));
}

function bottomValues(param, ary2d) {
  var y = param.y;
  if (y >= (ary2d.height - 1 | 0)) {
    return [];
  }
  var x = param.x;
  return Utils.mapRange(y + 1 | 0, ary2d.height - 1 | 0, (function (n) {
                return get(x, n, ary2d);
              }));
}

var Array2d = {
  fromRows: fromRows,
  $$new: $$new,
  get: get,
  set: set,
  enumerate: enumerate,
  leftValues: leftValues,
  rightValues: rightValues,
  topValues: topValues,
  bottomValues: bottomValues
};

function loadTrees(input) {
  return fromRows(Js_array.map((function (line) {
                    return Js_array.map(Belt_Option.getExn, Js_array.map(Belt_Int.fromString, Js_string.split("", line)));
                  }), Js_string.split(Os.EOL, input)));
}

function part01(input) {
  var trees = loadTrees(input);
  return Js_array.reduce((function (accu, cell) {
                var y = cell.y;
                var x = cell.x;
                if (x === 0 || x === (trees.width - 1 | 0) || y === 0 || y === (trees.height - 1 | 0)) {
                  return accu + 1 | 0;
                }
                var minMax = Caml_splice_call.spliceApply(Math.min, [Js_array.map((function (prim) {
                              return Caml_splice_call.spliceApply(Math.max, [prim]);
                            }), Js_array.filter((function (ary) {
                                  return ary.length > 0;
                                }), [
                                leftValues(cell, trees),
                                rightValues(cell, trees),
                                topValues(cell, trees),
                                bottomValues(cell, trees)
                              ]))]);
                if (minMax >= cell.val) {
                  return accu;
                } else {
                  return accu + 1 | 0;
                }
              }), 0, enumerate(trees));
}

function countUntilBiggerOrEqual(val, values) {
  return Js_array.reduce((function (accu, currentVal) {
                  if (accu[1]) {
                    return accu;
                  }
                  var count = accu[0];
                  if (currentVal < val) {
                    return [
                            count + 1 | 0,
                            false
                          ];
                  } else {
                    return [
                            count + 1 | 0,
                            true
                          ];
                  }
                }), [
                0,
                false
              ], values)[0];
}

function part02(input) {
  var trees = loadTrees(input);
  return Caml_splice_call.spliceApply(Math.max, [Js_array.map((function (cell) {
                      var partial_arg = cell.val;
                      return Js_array.reduce((function (prim0, prim1) {
                                    return Math.imul(prim0, prim1);
                                  }), 1, Js_array.map((function (param) {
                                        return countUntilBiggerOrEqual(partial_arg, param);
                                      }), [
                                      Utils.reverseArray(leftValues(cell, trees)),
                                      rightValues(cell, trees),
                                      Utils.reverseArray(topValues(cell, trees)),
                                      bottomValues(cell, trees)
                                    ]));
                    }), enumerate(trees))]);
}

export {
  Array2d ,
  loadTrees ,
  part01 ,
  countUntilBiggerOrEqual ,
  part02 ,
}
/* os Not a pure module */
