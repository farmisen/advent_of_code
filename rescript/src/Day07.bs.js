// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Os from "os";
import * as Utils from "./Utils.bs.js";
import * as Belt_Int from "rescript/lib/es6/belt_Int.js";
import * as Js_array from "rescript/lib/es6/js_array.js";
import * as Js_string from "rescript/lib/es6/js_string.js";
import * as Caml_array from "rescript/lib/es6/caml_array.js";

function emptyFs(param) {
  var root_children = [];
  var root = {
    name: "/",
    size: -1,
    parent: undefined,
    children: root_children
  };
  return {
          root: root,
          folders: [root],
          currentFolder: undefined
        };
}

function newFolder(name, parent) {
  return {
          name: name,
          size: -1,
          parent: parent,
          children: []
        };
}

function newFile(name, size, parent) {
  return {
          name: name,
          size: size,
          parent: parent
        };
}

function calcSize(folder) {
  return Js_array.reduce((function (accu, child) {
                if (child.TAG === /* Folder */0) {
                  return accu + calcSize(child._0) | 0;
                } else {
                  return accu + child._0.size | 0;
                }
              }), 0, folder.children);
}

function cd(name, fs) {
  if (name === "..") {
    var currentFolder = fs.currentFolder;
    if (currentFolder !== undefined) {
      var parentFolder = currentFolder.parent;
      if (parentFolder !== undefined) {
        return {
                root: fs.root,
                folders: fs.folders,
                currentFolder: parentFolder
              };
      }
      throw {
            RE_EXN_ID: Utils.WentSouth,
            Error: new Error()
          };
    }
    throw {
          RE_EXN_ID: Utils.WentSouth,
          Error: new Error()
        };
  }
  if (name === fs.root.name) {
    return {
            root: fs.root,
            folders: fs.folders,
            currentFolder: fs.root
          };
  }
  var folder = fs.currentFolder;
  if (folder !== undefined) {
    var maybeFldr = Js_array.find((function (child) {
            if (child.TAG === /* Folder */0) {
              return child._0.name === name;
            } else {
              return false;
            }
          }), folder.children);
    if (maybeFldr !== undefined) {
      if (maybeFldr.TAG === /* Folder */0) {
        return {
                root: fs.root,
                folders: fs.folders,
                currentFolder: maybeFldr._0
              };
      }
      throw {
            RE_EXN_ID: Utils.WentSouth,
            Error: new Error()
          };
    }
    throw {
          RE_EXN_ID: Utils.WentSouth,
          Error: new Error()
        };
  }
  throw {
        RE_EXN_ID: Utils.WentSouth,
        Error: new Error()
      };
}

function mkDir(name, fs) {
  var folder = fs.currentFolder;
  var parent;
  if (folder !== undefined) {
    parent = folder;
  } else {
    throw {
          RE_EXN_ID: Utils.WentSouth,
          Error: new Error()
        };
  }
  var maybeFldr = Js_array.find((function (child) {
          if (child.TAG === /* Folder */0) {
            return child._0.name === name;
          } else {
            return false;
          }
        }), parent.children);
  if (maybeFldr !== undefined) {
    if (maybeFldr.TAG !== /* Folder */0) {
      throw {
            RE_EXN_ID: Utils.WentSouth,
            Error: new Error()
          };
    }
    
  } else {
    var folder$1 = newFolder(name, parent);
    Utils.pushTo(parent.children, {
          TAG: /* Folder */0,
          _0: folder$1
        });
    Utils.pushTo(fs.folders, folder$1);
  }
  return fs;
}

function mkFile(size, name, fs) {
  var folder = fs.currentFolder;
  var parentFolder;
  if (folder !== undefined) {
    parentFolder = folder;
  } else {
    throw {
          RE_EXN_ID: Utils.WentSouth,
          Error: new Error()
        };
  }
  var number = Belt_Int.fromString(size);
  if (number !== undefined) {
    var file = newFile(name, number, parentFolder);
    Utils.pushTo(parentFolder.children, {
          TAG: /* File */1,
          _0: file
        });
  } else {
    throw {
          RE_EXN_ID: Utils.WentSouth,
          Error: new Error()
        };
  }
  return fs;
}

var Fs = {
  emptyFs: emptyFs,
  newFolder: newFolder,
  newFile: newFile,
  calcSize: calcSize,
  cd: cd,
  mkDir: mkDir,
  mkFile: mkFile
};

function loadFs(input) {
  return Js_array.reduce((function (fs, cmd) {
                return Js_array.reduce((function (fs, line) {
                              var tokens = Js_string.split(" ", line);
                              if (Js_string.startsWith("cd", Caml_array.get(tokens, 0))) {
                                return cd(Caml_array.get(tokens, 1), fs);
                              } else if (Js_string.startsWith("dir", Caml_array.get(tokens, 0))) {
                                return mkDir(Caml_array.get(tokens, 1), fs);
                              } else if (tokens.length === 2) {
                                return mkFile(Caml_array.get(tokens, 0), Caml_array.get(tokens, 1), fs);
                              } else {
                                return fs;
                              }
                            }), fs, Js_string.split(Os.EOL, cmd));
              }), emptyFs(undefined), Js_array.map((function (prim) {
                    return prim.trim();
                  }), Js_string.split("$", input)));
}

function part01(input) {
  var fs = loadFs(input);
  return Js_array.reduce((function (prim0, prim1) {
                return prim0 + prim1 | 0;
              }), 0, Js_array.filter((function (size) {
                    return size <= 100000;
                  }), Js_array.map(calcSize, fs.folders)));
}

function part02(input) {
  var fs = loadFs(input);
  var used_space = calcSize(fs.root);
  var needed_space = 30000000 - (70000000 - used_space | 0) | 0;
  return Utils.first(Utils.sortArrayWith((function (a, b) {
                    return a - b | 0;
                  }), Js_array.filter((function (size) {
                        return size >= needed_space;
                      }), Js_array.map(calcSize, fs.folders))));
}

export {
  Fs ,
  loadFs ,
  part01 ,
  part02 ,
}
/* os Not a pure module */
