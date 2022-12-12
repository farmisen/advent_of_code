open Js.Array
open Utils
open Belt.Set.String

let toCompartments = line => {
  let midPoint = Js.String.length(line) / 2
  [
    line |> Js.String.slice(~from=0, ~to_=midPoint),
    line |> Js.String.sliceToEnd(~from=midPoint),
  ] |> map(str => str |> Js.String.split("") |> Belt.Set.String.fromArray(_))
}

let priority = c =>
  switch c {
  | lowerChar if lowerChar >= "a" && lowerChar <= "z" =>
    lowerChar |> Js.String.charCodeAt(0) |> Belt.Float.toInt |> (n => n - 97 + 1)
  | upperChar if upperChar >= "A" && upperChar <= "Z" =>
    upperChar |> Js.String.charCodeAt(0) |> Belt.Float.toInt |> (n => n - 65 + 27)
  | _ => raise(WentSouth)
  }

let part01 = () =>
  loadLines(3)
  |> map(toCompartments)
  |> map(compartments => {
    compartments[0]->intersect(compartments[1]) |> Belt.Set.String.toArray(_) |> first |> priority
  })
  |> Js.Array.reduce(\"+", 0)

let part02 = () => {
  loadLines(3)
  |> toChunks(3)
  |> map(chunk =>
    chunk |> map(rucksack => rucksack |> Js.String.split("") |> Belt.Set.String.fromArray(_))
  )
  |> map(chunk => {
    switch chunk {
    | [rucksack1, rucksack2, rucksack3] =>
      rucksack1->intersect(rucksack2)->intersect(rucksack3)
      |> Belt.Set.String.toArray(_)
      |> first
      |> priority
    | _ => raise(WentSouth)
    }
  })
  |> Js.Array.reduce(\"+", 0)
}
