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

let first = (arr: array<'a>): 'a => {
  switch length(arr) {
  | 0 => raise(WentSouth)
  | _ => arr[0]
  }
}

let last = (arr: array<'a>): 'a =>
  switch length(arr) {
  | 0 => raise(WentSouth)
  | _ => arr[length(arr)]
  }

let loadLines = (day: int) => day |> loadInput |> Js.String.split(Os.eol)

let pushTo = (arr: array<'a>, item: 'a): array<'a> => {
  push(item, arr)->ignore
  arr
}

let unshiftArray = (arr: array<'a>, item: 'a): array<'a> => {
  unshift(item, arr)->ignore
  arr
}
let reverseArray = (arr: array<'a>): array<'a> => {
  reverseInPlace(arr)->ignore
  arr
}

let sortArrayWith = (with: (int, int) => int, arr: array<'a>): array<'a> => {
  sortInPlaceWith(with, arr)->ignore
  arr
}

let sliceArrayAt = (idx: int, arr: array<'a>): (array<'a>, array<'a>) => {
  (slice(~start=0, ~end_=idx, arr), sliceFrom(idx, arr))
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

let arrayOfSize = (size: int, valueAt: int => 'a): array<'a> => {
  let arr = []
  for idx in 0 to size - 1 {
    push(valueAt(idx), arr)->ignore
  }
  arr
}
