import { assert } from "console";
import DataHelpers, { Point3 } from "./data/dataHelpers.js";

namespace d22 {
  enum Action {
    On,
    Off,
  }
  class CubeAction {
    constructor(action: Action, start: Point3, stop: Point3) {
      this.action = action;
      this.start = start;
      this.stop = stop;
    }
    public action: Action;
    public start: Point3;
    public stop: Point3;
    public contains(p: Point3): boolean {
      return (
        p.x >= this.start.x &&
        p.x <= this.stop.x &&
        p.y >= this.start.y &&
        p.y <= this.stop.y &&
        p.z >= this.start.z &&
        p.z <= this.stop.z
      );
    }
  }

  function rangeFromLine(line: string): { start: number; end: number } {
    let nums = line.split("..");
    assert(nums.length == 2);
    let start = parseInt(nums[0]);
    let end = parseInt(nums[1]);
    assert(start <= end);
    return { start: start, end: end };
  }

  function actionFromLine(line: string): CubeAction {
    let parts = line.split(new RegExp("[ ,=]+"));
    assert(parts.length == 7);
    let action = parts[0] == "on" ? Action.On : Action.Off;
    assert(parts[1] == "x" && parts[3] == "y" && parts[5] == "z");
    let range_x = rangeFromLine(parts[2]);
    let range_y = rangeFromLine(parts[4]);
    let range_z = rangeFromLine(parts[6]);
    let ret = new CubeAction(
      action,
      new Point3(range_x.start, range_y.start, range_z.start),
      new Point3(range_x.end, range_y.end, range_z.end)
    );
    return ret;
  }

  function loadData(filename: string): CubeAction[] {
    var lines: string[] = DataHelpers.fileToStringArray(filename);
    return lines
      .map((line) => actionFromLine(line))
      .slice()
      .reverse();
  }

  function countOn(actions: CubeAction[], start: Point3, stop: Point3): number {
    let on_count = 0;
    // let total_points_tested = (stop.x - start.x + 1) * (stop.y - start.y + 1) * (stop.z - start.z + 1);
    // console.log(`Testing ${total_points_tested}`);
    for (let z = start.z; z <= stop.z; z++) {
      for (let y = start.y; y <= stop.y; y++) {
        for (let x = start.x; x <= stop.x; x++) {
          let p = new Point3(x, y, z);
          let on = false;
          for (let action of actions) {
            if (action.contains(p)) {
              if (action.action == Action.On) {
                on_count++;
              }
              break;
            }
          }
        }
      }
    }
    return on_count;
  }

  export function part1(desc: string, filename: string) {
    var actions = loadData(filename);
    let on_count = countOn(actions, new Point3(-50, -50, -50), new Point3(50, 50, 50));
    console.log(`${desc}: ${on_count}`);
  }

  export function part2(desc: string, filename: string) {
    var actions = loadData(filename);
    let min = actions
      .map((action) => action.start)
      .reduce((prev, current) => {
        return prev.min(current);
      });
    let max = actions
      .map((action) => action.stop)
      .reduce((prev, current) => {
        return prev.max(current);
      });
    let on_count = countOn(actions, min, max);
    console.log(`${desc}: ${on_count}`);
  }
}

export function solve() {
  console.log(`Day 22:`);
  d22.part1("P1 (test)", "./data/d22_test_1.txt");
  d22.part1("P1 (real)", "./data/d22_real.txt");
  d22.part1("P1 (test2)", "./data/d22_test_2.txt");
  d22.part2("P2 (test2)", "./data/d22_test_2.txt");
  //d22.part2("P2 (real)", "./data/d22_real.txt");
}

// Day 22:
// P1 (test): 590784
// P1 (real): 542711
// P1 (test2): 474140
// 14125876564443248
//  2758514936282235
