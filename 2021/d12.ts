import DataHelpers from "./data/dataHelpers.js";

namespace d12 {
  class Connection {
    a: string;
    b: string;
    constructor(a: string, b: string) {
      this.a = a;
      this.b = b;
    }
  }

  function findPathToEnd(connections: Connection[], path: string[], currNode: string): number {
    let count: number = 0;
    if (currNode == "end") {
      //console.log(path.reduce((p, c) => p + "," + c) + ",end");
      return 1;
    }
    // part 1
    // if (currNode.toLowerCase() == currNode && path.findIndex(p => p == currNode) >= 0) {
    //   return 0;
    // }
    // part 2.  Gross
    if (currNode.toLowerCase() == currNode) {
      let needReturn: boolean = false;
      let currLetterCount: number = path.filter((p) => p == currNode).length;
      if (currLetterCount >= 2) {
        // Can't go to three
        needReturn = true;
      }
      if (currLetterCount == 1) {
        // Can to 2, but only if there are no other twos
        path
          .filter((p) => p.toLowerCase() == p)
          .forEach((p) => {
            if (path.filter((p2) => p2 == p).length > 1) {
              needReturn = true;
            }
          });
      }
      if (needReturn) {
        return 0;
      }
    }
    path.push(currNode);
    connections
      .filter((c) => c.a == currNode)
      .forEach((c) => {
        count += findPathToEnd(connections, path, c.b);
      });
    path.pop();
    return count;
  }

  export function part1(desc: string, filename: string) {
    let pathStrrings: string[] = DataHelpers.fileToStringArray(filename);
    let connections: Connection[] = new Array<Connection>();

    pathStrrings.forEach((s) => {
      let locs: string[] = s.split("-");
      // Can't move to start or from end
      if (locs[1] != "start" && locs[0] != "end") {
        connections.push(new Connection(locs[0], locs[1]));
      }
      // Other way
      if (locs[0] != "start" && locs[1] != "end") {
        connections.push(new Connection(locs[1], locs[0]));
      }
    });

    console.log(connections);

    let path: string[] = new Array<string>();
    let pathCount = findPathToEnd(connections, path, "start");
    console.log(`P1 Total Connections (${desc}) = ${pathCount}`);
  }

  export function part2(desc: string, filename: string) {
    let ret: number = 0;
    console.log(`P2 Score (${desc}) = ${ret}`);
  }
}

d12.part1("test", "./data/d12_real.txt");
//d12.part1("real", './data/d12_real.txt');
// d12.part2("test", './data/d12_test.txt');
// d12.part2("real", './data/d12_real.txt');
