// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Os from "os";
import * as Utils from "./Utils.bs.js";
import * as Belt_Int from "rescript/lib/es6/belt_Int.js";
import * as Js_array from "rescript/lib/es6/js_array.js";
import * as Js_string from "rescript/lib/es6/js_string.js";
import * as Caml_array from "rescript/lib/es6/caml_array.js";
import * as Belt_Option from "rescript/lib/es6/belt_Option.js";

function makeRope(len) {
  return Utils.mapRange(0, len - 1 | 0, (function (param) {
                return {
                        x: 0,
                        y: 0
                      };
              }));
}

function sign(n) {
  if (n !== 0) {
    if (n < 0) {
      return -1;
    } else {
      return 1;
    }
  } else {
    return 0;
  }
}

function loadMoves(input) {
  return Js_array.map((function (tokens) {
                var count = Belt_Option.getExn(Belt_Int.fromString(Caml_array.get(tokens, 1)));
                var match = Caml_array.get(tokens, 0);
                switch (match) {
                  case "D" :
                      return {
                              dx: 0,
                              dy: -1,
                              count: count
                            };
                  case "L" :
                      return {
                              dx: -1,
                              dy: 0,
                              count: count
                            };
                  case "R" :
                      return {
                              dx: 1,
                              dy: 0,
                              count: count
                            };
                  case "U" :
                      return {
                              dx: 0,
                              dy: 1,
                              count: count
                            };
                  default:
                    throw {
                          RE_EXN_ID: Utils.WentSouth,
                          Error: new Error()
                        };
                }
              }), Js_array.map((function (line) {
                    return Js_string.split(" ", line);
                  }), Js_string.split(Os.EOL, input)));
}

function updateRope(move, state) {
  var visited = state.visited;
  var newRope = Js_array.reduce((function (knots, knot) {
          var match = knots.length;
          var newKnot;
          if (match !== 0) {
            var prev = Utils.last(knots);
            var dx = prev.x - knot.x | 0;
            var dy = prev.y - knot.y | 0;
            var match$1 = Math.abs(dx);
            var match$2 = Math.abs(dy);
            var exit = 0;
            if (match$1 <= 1 && match$2 <= 1) {
              newKnot = knot;
            } else {
              switch (match$1) {
                case 0 :
                    if (match$2 !== 2) {
                      exit = 1;
                    } else {
                      newKnot = {
                        x: knot.x,
                        y: knot.y + sign(dy) | 0
                      };
                    }
                    break;
                case 1 :
                    exit = 1;
                    break;
                case 2 :
                    if (match$2 !== 0) {
                      exit = 1;
                    } else {
                      newKnot = {
                        x: knot.x + sign(dx) | 0,
                        y: knot.y
                      };
                    }
                    break;
                default:
                  exit = 1;
              }
            }
            if (exit === 1) {
              newKnot = {
                x: knot.x + sign(dx) | 0,
                y: knot.y + sign(dy) | 0
              };
            }
            
          } else {
            newKnot = {
              x: knot.x + move.dx | 0,
              y: knot.y + move.dy | 0
            };
          }
          return Utils.pushTo(knots, newKnot);
        }), [], state.rope);
  var tail = Utils.last(newRope);
  var match = Js_array.find((function (pos) {
          if (pos.x === tail.x) {
            return pos.y === tail.y;
          } else {
            return false;
          }
        }), visited);
  if (match !== undefined) {
    return {
            rope: newRope,
            visited: visited
          };
  } else {
    return {
            rope: newRope,
            visited: Utils.pushTo(visited, tail)
          };
  }
}

function simulate(len, moves) {
  var match = Js_array.reduce((function (state, move) {
          var _param = move;
          var _state = state;
          while(true) {
            var param = _param;
            var state$1 = _state;
            var count = param.count;
            if (count === 0) {
              return state$1;
            }
            _state = updateRope(move, state$1);
            _param = {
              dx: param.dx,
              dy: param.dy,
              count: count - 1 | 0
            };
            continue ;
          };
        }), {
        rope: makeRope(len),
        visited: []
      }, moves);
  return match.visited.length;
}

function part01(input) {
  return simulate(2, loadMoves(input));
}

function part02(input) {
  return simulate(10, loadMoves(input));
}

export {
  makeRope ,
  sign ,
  loadMoves ,
  updateRope ,
  simulate ,
  part01 ,
  part02 ,
}
/* os Not a pure module */
