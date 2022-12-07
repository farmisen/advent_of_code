open NodeJs
open Js.Array
open Js.String
open Utils

let run = () => {
  loadInput(1)
  |> split(Os.eol ++ Os.eol)
  |> map(
    split(Os.eol) |> map(Belt.Int.fromString) |> map(unwrapOrRaise(WentSouth)) |> reduce(\"+", 0),
  )
  |> sortNumbers
  |> pop
  |> unwrapOrRaise(WentSouth)
}
