open Js.String
open Js.Array
open Utils

module Fs = {
  type rec folder = {
    name: string,
    size: int,
    parent: option<folder>,
    children: array<fsTree>,
  }
  and file = {name: string, size: int, parent: option<folder>}
  and fsTree =
    | Folder(folder)
    | File(file)

  type fs = {
    root: folder,
    folders: array<folder>,
    currentFolder: option<folder>,
  }

  let emptyFs = (): fs => {
    let root = {name: "/", size: -1, children: [], parent: None}
    {root, currentFolder: None, folders: [root]}
  }

  let newFolder = (name: string, parent: folder): folder => {
    {name, size: -1, children: [], parent: Some(parent)}
  }

  let newFile = (name: string, size: int, parent: folder): file => {
    {name, size, parent: Some(parent)}
  }

  let rec calcSize = (folder: folder): int => {
    folder.children |> reduce((accu: int, child: fsTree): int => {
      switch child {
      | Folder(childFolder) => accu + (childFolder |> calcSize)
      | File(file) => accu + file.size
      }
    }, 0)
  }

  let cd = (name: string, fs: fs): fs => {
    switch name {
    | ".." =>
      switch fs.currentFolder {
      | None => raise(WentSouth)
      | Some(currentFolder) =>
        switch currentFolder.parent {
        | None => raise(WentSouth)
        | Some(parentFolder) => {...fs, currentFolder: Some(parentFolder)}
        }
      }

    | _ if name === fs.root.name => {...fs, currentFolder: Some(fs.root)}
    | _ =>
      switch fs.currentFolder {
      | None => raise(WentSouth)
      | Some(folder) => {
          let maybeFldr = folder.children |> find(child =>
            switch child {
            | Folder(fldr) => fldr.name === name
            | _ => false
            }
          )

          switch maybeFldr {
          | Some(Folder(folder)) => {...fs, currentFolder: Some(folder)}
          | _ => raise(WentSouth)
          }
        }
      }
    }
  }

  let mkDir = (name: string, fs: fs): fs => {
    let parent = switch fs.currentFolder {
    | Some(folder) => folder
    | None => raise(WentSouth)
    }

    let maybeFldr = parent.children |> find(child =>
      switch child {
      | Folder(f) => f.name === name
      | File(_) => false
      }
    )

    switch maybeFldr {
    | Some(Folder(_)) => ()
    | None => {
        let folder = newFolder(name, parent)
        (Folder(folder) |> pushTo(parent.children))->ignore
        (folder |> pushTo(fs.folders))->ignore
      }

    | _ => raise(WentSouth)
    }
    fs
  }

  let mkFile = (size: string, name: string, fs: fs): fs => {
    let parentFolder = switch fs.currentFolder {
    | Some(folder) => folder
    | None => raise(WentSouth)
    }

    switch size |> Belt.Int.fromString {
    | Some(number) => {
        let file = newFile(name, number, parentFolder)
        (File(file) |> pushTo(parentFolder.children))->ignore
      }

    | None => raise(WentSouth)
    }

    fs
  }
}

let loadFs = (input: string): Fs.fs => {
  let fs = input |> split("$") |> map(trim) |> reduce((fs: Fs.fs, cmd: string): Fs.fs => {
      cmd |> split(NodeJs.Os.eol) |> reduce((fs: Fs.fs, line: string): Fs.fs => {
        let tokens = line |> split(" ")

        switch tokens[0] {
        | _ if tokens[0] |> startsWith("cd") => fs |> Fs.cd(tokens[1])
        | _ if tokens[0] |> startsWith("dir") => fs |> Fs.mkDir(tokens[1])
        | _ if tokens |> Js.Array.length === 2 => fs |> Fs.mkFile(tokens[0], tokens[1])
        | _ => fs
        }
      }, fs)
    }, Fs.emptyFs())
  fs
}

let part01 = (input: string): int => {
  let fs = input |> loadFs
  
  fs.folders
  |> map(folder => folder |> Fs.calcSize)
  |> filter((size: int): bool => size <= 100000)
  |> reduce(\"+", 0)
}

let part02 = (input: string): int => {
  let fs = input |> loadFs

  let total_disk_space = 70000000
  let required_space = 30000000
  let used_space = fs.root |> Fs.calcSize
  let needed_space = required_space - (total_disk_space - used_space)

  fs.folders
  |> map(folder => folder |> Fs.calcSize)
  |> filter((size: int): bool => size >= needed_space)
  |> sortArrayWith((a: int, b: int): int => a - b)
  |> first
}
