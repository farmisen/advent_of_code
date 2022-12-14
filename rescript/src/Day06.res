open Js.String

let findMarkerOfSize = (size: int, packets: string): int => {
  (packets
  |> split("")
  |> Js.Array.findIndexi((_, idx) =>
    packets
    |> slice(~from=idx, ~to_=idx + size)
    |> split("")
    |> Belt.Set.String.fromArray
    |> Belt.Set.String.size == size
  )) + size
}

let part01 = (input: string): int => {
  input |> findMarkerOfSize(4)
}

let part02 = (input: string): int => {
  input |> findMarkerOfSize(14)
}
