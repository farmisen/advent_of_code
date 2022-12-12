open Js.Array
open Utils

let includeOther = (r0, r1) => r0[0] <= r1[0] && r0[1] >= r1[1]
let includeAny = (r0, r1) => includeOther(r0, r1) || includeOther(r1, r0)
let overlap = (r0, r1) => (r0[0] >= r1[0] && r0[0] <= r1[1]) || (r1[0] >= r0[0] && r1[0] <= r0[1])

let toRanges = line =>
  line
  |> Js.String.split(",")
  |> map(range => range |> Js.String.split("-") |> map(str => Belt.Int.fromString(str)))

let part01 = () => {
  loadLines(4)
  |> map(toRanges)
  |> filter(ranges =>
    switch ranges {
    | [r0, r1] => includeAny(r0, r1)
    | _ => false
    }
  )
  |> length
}

let part02 = () => {
  loadLines(4)
  |> map(toRanges)
  |> filter(ranges =>
    switch ranges {
    | [r0, r1] => overlap(r0, r1)
    | _ => false
    }
  )
  |> length
}
