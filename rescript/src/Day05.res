open Js.Array
open Utils

type move = {from: int, to: int, count: int}
type context = {stacks: array<array<string>>, moves: array<move>}

let loadContext = (): context => {
  loadLines(5) |> reduce((accu: context, line: string) => {
    switch line {
    // handle stacks initial state line
    | stacksLine if stacksLine |> Js.String.trim |> Js.String.startsWith("[") =>
      Belt.Range.forEach(0, 8, idx =>
        switch stacksLine |> Js.String.charAt(idx * 4 + 1) {
        | " " => ()
        | c => (c |> unshiftArray(accu.stacks[idx]))->ignore
        }
      )

    // handle move line
    | moveLine if moveLine |> Js.String.startsWith("move") =>
      (switch moveLine
      |> Js.String.split(" ")
      |> map(Belt.Int.fromString)
      |> filter(Belt.Option.isSome) {
      | [Some(count), Some(from), Some(to)] => {count, from, to}
      | _ => raise(WentSouth)
      } |> pushTo(accu.moves))->ignore

    // don't do anything for other lines
    | _ => ()
    }
    accu
  }, {stacks: arrayOfSize(9, _ => []), moves: []})
}

let part01 = () => {
  let {stacks, moves} = loadContext()
  moves |> forEach(({from, to, count}) => {
    let (leftOver, chunk) = stacks[from - 1] |> sliceArrayAt(length(stacks[from - 1]) - count)
    stacks[from - 1] = leftOver
    stacks[to - 1] = stacks[to - 1] |> concat(chunk |> reverseArray)
  })

  stacks |> map(stack => stack |> pop |> Belt.Option.getExn)
}

let part02 = () => {
  let {stacks, moves} = loadContext()
  moves |> forEach(({from, to, count}) => {
    let (leftOver, chunk) = stacks[from - 1] |> sliceArrayAt(length(stacks[from - 1]) - count)
    stacks[from - 1] = leftOver
    stacks[to - 1] = stacks[to - 1] |> concat(chunk)
  })

  stacks |> map(stack => stack |> pop |> Belt.Option.getExn)
}
