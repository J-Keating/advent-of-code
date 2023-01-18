import { buffer } from "stream/consumers";
import { PointXY } from "./data/dataHelpers.js";

namespace d17 {
  export function part1(desc: string, filename: string) {}

  export function part2(desc: string, top_left: PointXY, bottom_right: PointXY) {
    let start_loc: PointXY = new PointXY(0, 0);
    let hit_count: number = 0;

    // for (var start_y_vel = top_left.y + 35; start_y_vel >= bottom_right.y - 35; start_y_vel--) {
    //   let line: String;
    //   for (var start_x_vel = top_left.x - 35; start_x_vel <= bottom_right.x + 35; start_x_vel++) {
    for (var start_y_vel = 4095; start_y_vel >= bottom_right.y - 35; start_y_vel--) {
      let line: String;
      for (var start_x_vel = 1; start_x_vel <= bottom_right.x + 35; start_x_vel++) {
        //let special = start_x_vel == 6 && start_y_vel == 3;
        if (start_x_vel == 0 && start_y_vel == 0) {
          line += "O";
          continue;
        }
        let loc: PointXY = new PointXY(start_loc.x, start_loc.y);
        let vel: PointXY = new PointXY(start_x_vel, start_y_vel);
        let hit: boolean = false;
        while (true) {
          loc.add(vel);
          //if (special) {
          //line += "*";
          //}
          if (top_left.x <= loc.x && loc.x <= bottom_right.x && top_left.y >= loc.y && loc.y >= bottom_right.y) {
            hit = true;
            break;
          } else if (loc.x > bottom_right.x || loc.y < bottom_right.y) {
            break;
          } else {
            if (vel.x > 0) vel.x -= 1;
            vel.y -= 1;
          }
        }
        {
          if (hit) {
            hit_count++;
            line += "#";
            //console.log(`${start_x_vel},${start_y_vel}`);
          } else {
            line += ".";
          }
        }
      }
      //console.log(line);
    }

    console.log(`Total (${desc}) = ${hit_count}`);
  }
}

export function solve() {
  //d17.part2("test", new PointXY(20, -5), new PointXY(30, -10));
  d17.part2("real", new PointXY(244, -54), new PointXY(303, -91));
}
