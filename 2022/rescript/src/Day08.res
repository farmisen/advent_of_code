open Js.String
open Js.Array
open Utils

module Array2d = {
  type cell<'a> = {
    x: int,
    y: int,
    val: 'a,
  }

  type array2d<'a> = {
    width: int,
    height: int,
    elements: array<'a>,
  }

  let fromRows = (rows: array<array<'a>>): array2d<'a> => {
    width: rows[0] |> Js.Array.length,
    height: rows |> Js.Array.length,
    elements: [] |> Js.Array.concatMany(rows),
  }

  let new = (width: int, height: int, val: 'a): array2d<'a> => {
    let rows = mapRange(0, height - 1, _ => mapRange(0, width - 1, _ => val))
    fromRows(rows)
  }

  let get = (x: int, y: int, ary2d: array2d<'a>): 'a => {
    ary2d.elements[x + y * ary2d.width]
  }

  let set = (x: int, y: int, val: int, ary2d: array2d<'a>): array2d<'a> => {
    ary2d.elements[x + y * ary2d.width] = val
    ary2d
  }

  let enumerate = (ary2d: array2d<'a>): array<cell<'a>> =>
    ary2d.elements |> mapi((val, idx) => {
      x: mod(idx, ary2d.width),
      y: idx / ary2d.height,
      val,
    })

  let leftValues = ({x, y, _}: cell<'a>, ary2d: array2d<'a>): array<'a> =>
    switch x {
    | _ if x <= 0 => []
    | _ => mapRange(0, x - 1, n => ary2d |> get(n, y))
    }

  let rightValues = ({x, y, _}: cell<'a>, ary2d: array2d<'a>): array<'a> =>
    switch x {
    | _ if x >= ary2d.width - 1 => []
    | _ => mapRange(x + 1, ary2d.width - 1, n => ary2d |> get(n, y))
    }

  let topValues = ({x, y, _}: cell<'a>, ary2d: array2d<'a>): array<'a> =>
    switch y {
    | _ if y <= 0 => []
    | _ => mapRange(0, y - 1, n => ary2d |> get(x, n))
    }

  let bottomValues = ({x, y, _}: cell<'a>, ary2d: array2d<'a>): array<'a> =>
    switch y {
    | _ if y >= ary2d.height - 1 => []
    | _ => mapRange(y + 1, ary2d.height - 1, n => ary2d |> get(x, n))
    }
}

let loadTrees = input =>
  input
  |> split(NodeJs.Os.eol)
  |> map(line => line |> split("") |> map(Belt.Int.fromString) |> map(Belt.Option.getExn))
  |> Array2d.fromRows

let part01 = (input: string): int => {
  let trees = input |> loadTrees

  trees |> Array2d.enumerate |> reduce((accu, cell: Array2d.cell<int>) => {
    switch cell {
    | {x, y} if x == 0 || x == trees.width - 1 || y == 0 || y == trees.height - 1 => accu + 1
    | _ => {
        switch 
          [
            trees |> Array2d.leftValues(cell),
            trees |> Array2d.rightValues(cell),
            trees |> Array2d.topValues(cell),
            trees |> Array2d.bottomValues(cell),
          ]
          |> filter(ary => Js.Array.length(ary) > 0)
          |> map(Js.Math.maxMany_int)
          |> Js.Math.minMany_int {
            | minMax if minMax >= cell.val => accu
            | _ =>accu + 1
          }
      }
    }
  }, 0)
}

let countUntilBiggerOrEqual = (val: int, values: array<int>): int => {
  let (count, _) = values |> reduce((accu, currentVal) => {
    switch accu {
    | (_, true) => accu
    | (count, false) if currentVal < val => (count + 1, false)
    | (count, false) => (count + 1, true)
    }
  }, (0, false))
  count
}

let part02 = (input: string): int => {
  let trees = input |> loadTrees

  trees
  |> Array2d.enumerate
  |> map(cell => {
    [
      trees |> Array2d.leftValues(cell) |> Utils.reverseArray,
      trees |> Array2d.rightValues(cell),
      trees |> Array2d.topValues(cell) |> Utils.reverseArray,
      trees |> Array2d.bottomValues(cell),
    ]
    |> map(countUntilBiggerOrEqual(cell.val))
    |> reduce(\"*", 1)
  })
  |> Js.Math.maxMany_int
}
