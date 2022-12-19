open Js.Array
open Utils

type instruction =
  | Noop
  | Addx(int)

type cpu = {
  x: int,
  pw: int,
  crt: string,
  cycle: int,
}

let newCpu = () => {x: 1, cycle: 1, pw: 0, crt: ""}

let toInstructions = (input: string): array<instruction> =>
  input
  |> Js.String.split(NodeJs.Os.eol)
  |> filter(line => line !== "")
  |> map(line => line |> Js.String.split(" "))
  |> map(tokens => {
    switch tokens[0] {
    | _ if tokens[0] === "noop" => Noop
    | _ if tokens[0] === "addx" => Addx(tokens[1] |> Belt.Int.fromString |> Belt.Option.getExn)
    | _ => raise(WentSouth)
    }
  })

let execute = (ins: instruction, cpu: cpu): cpu =>
  switch ins {
  | Noop => cpu
  | Addx(val) => {...cpu, x: cpu.x + val}
  }

let updatePw = (cpu: cpu): cpu =>
  switch mod(cpu.cycle - 20, 40) {
  | 0 => {...cpu, pw: cpu.pw + cpu.cycle * cpu.x}
  | _ => cpu
  }

let nextCycle = (cpu: cpu): cpu => {...cpu, cycle: cpu.cycle + 1}



let updateCrt = (cpu: cpu): cpu =>
  switch Js.Math.abs_int(mod(cpu.cycle - 1, 40) - cpu.x) <= 1 {
  | false => {...cpu, crt: cpu.crt ++ "."}
  | _ => {...cpu, crt: cpu.crt ++ "#"}
  }

let part01 = (input: string): int => {
  let {pw} = input |> toInstructions |> reduce((cpu, ins) => {
      switch ins {
      | Noop => cpu |> updatePw |> execute(ins) |> nextCycle
      | Addx(_) => cpu |> updatePw |> nextCycle |> updatePw |> execute(ins) |> nextCycle
      }
    }, newCpu())

  pw
}

let part02 = (input: string): string => {
  let {crt} = input |> toInstructions |> reduce((cpu, ins) => {
      switch ins {
      | Noop => cpu |> updateCrt |> execute(ins) |> nextCycle
      | Addx(_) => cpu |> updateCrt |> nextCycle |> updateCrt |> execute(ins) |> nextCycle
      }
    }, newCpu())

  mapRange(0, 5, row => {
    crt |> Js.String.slice(~from=row * 40, ~to_=(row + 1) * 40)
  }) |> joinWith("\n")
}
