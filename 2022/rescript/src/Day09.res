open Js.String
open Js.Array
open Utils

type vec2d = {
  x: int,
  y: int,
}

type move = {
  dx: int,
  dy: int,
  count: int,
}

type rope = array<vec2d>

type state = {
  rope: rope,
  visited: array<vec2d>,
}

let makeRope = (len: int): rope => mapRange(0, len - 1, _ => {x: 0, y: 0})

// TODO: find a better name
let sign = (n: int): int =>
  switch n {
  | 0 => 0
  | _ if n < 0 => -1
  | _ => 1
  }

let loadMoves = input =>
  input
  |> split(NodeJs.Os.eol)
  |> map(line => line |> split(" "))
  |> map(tokens => {
    let count = tokens[1] |> Belt.Int.fromString |> Belt.Option.getExn
    switch tokens[0] {
    | "L" => {dx: -1, dy: 0, count}
    | "R" => {dx: 1, dy: 0, count}
    | "U" => {dx: 0, dy: 1, count}
    | "D" => {dx: 0, dy: -1, count}
    | _ => raise(WentSouth)
    }
  })

let updateRope = (move: move, state: state): state => {
  let {rope, visited} = state

  let newRope = rope |> reduce((knots: array<vec2d>, knot: vec2d) => {
    let newKnot = switch knots |> Js.Array.length {
    | 0 => {x: knot.x + move.dx, y: knot.y + move.dy}
    | _ => {
        let prev = knots |> last
        let dx = prev.x - knot.x
        let dy = prev.y - knot.y

        switch (Js.Math.abs_int(dx), Js.Math.abs_int(dy)) {
        | (s, t) if s <= 1 && t <= 1 => knot
        | (0, 2) => {...knot, y: knot.y + sign(dy)}
        | (2, 0) => {...knot, x: knot.x + sign(dx)}
        | _ => {x: knot.x + sign(dx), y: knot.y + sign(dy)}
        }
      }
    }

    newKnot |> pushTo(knots)
  }, [])

  let tail = newRope |> last

  switch visited |> find((pos: vec2d): bool => pos.x === tail.x && pos.y === tail.y) {
  | Some(_) => {rope: newRope, visited}
  | None => {rope: newRope, visited: tail |> pushTo(visited)}
  }
}

let simulate = (len: int, moves: array<move>) => {
  let {visited} = moves |> reduce((state: state, move: move) => {
    let rec reduceMove = ({dx, dy, count}: move, state: state): state => {
      switch count {
      | 0 => state
      | _ => state |> updateRope(move) |> reduceMove({dx, dy, count: count - 1})
      }
    }
    state |> reduceMove(move)
  }, {rope: makeRope(len), visited: []})

  Js.Array.length(visited)
}

let part01 = (input: string): int => {
  input |> loadMoves |> simulate(2)
}

let part02 = (input: string): int => {
  input |> loadMoves |> simulate(10)
}
