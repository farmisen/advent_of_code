open NodeJs
open Js.Array

@send external padStart: (string, int, string) => string = "padStart"
@send external toString: int => string = "toString"

exception WentSouth

let loadInput = (day: int) => {
  let path = "../puzzles/day_" ++ day->toString->padStart(2, "0") ++ "/input.txt"
  Fs.readFileSyncWith(path, Fs.readFileOptions(~encoding="UTF-8", ()))->Buffer.toString
}

let sortNumbers = sortInPlaceWith((a, b) => a - b)

let unwrapOrRaise = (exp, a) => {
  switch a {
  | Some(thing) => thing
  | None => raise(exp)
  }
}

let first = array => array[0]

let loadLines = (day: int) => day |> loadInput |> Js.String.split(Os.eol)

let pushTo = (arr: array<'a>, item: 'a): array<'a> => {
  push(item, arr)->ignore
  arr
}

let toChunks = (size, arr) => {
  arr |> reducei((accumulator, item, index) => {
    let chunk = if mod(index, size) === 0 {
      []
    } else {
      pop(accumulator) |> Belt.Option.getWithDefault(_, [])
    }
    item |> pushTo(chunk) |> pushTo(accumulator)
  }, [])
}
