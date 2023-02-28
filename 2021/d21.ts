import { assert } from "console";
import d20Data from "./data/d20_data.js";
import DataHelpers from "./data/dataHelpers.js";

namespace d21 {
  class Die {
    private _state: number;
    constructor() {
      this._state = -1;
    }
    public roll(): number {
      this._state = (this._state + 1) % 100;
      return this._state + 1;
    }
    public roll_n_times(count: number): number {
      let ret = 0;
      for (let i = 0; i < count; i++) {
        ret += this.roll();
      }
      return ret;
      // Clever, but a bad idea, since map() calls roll(), which has a persistent side effect.
      // Reads better as a for loop.  And, it's slower.
      // return Array(count)
      //   .fill(0)
      //   .map((_) => this.roll())
      //   .reduce((a, b) => a + b);
    }
  }

  class QuantumDie {
    public static three_roll_options(): number[][] {
      return [
        [3, 1],
        [4, 3],
        [5, 6],
        [6, 7],
        [7, 6],
        [8, 3],
        [9, 1],
      ];
    }
  }

  export class Player {
    private _state: number;
    public index: number;
    public score: number;
    public count: number;
    constructor(position: number, index: number) {
      this._state = position - 1;
      this.index = index;
      this.score = 0;
      this.count = 1;
    }
    public get position(): number {
      return this._state + 1;
    }
    public move_by(steps: number) {
      this._state = (this._state + steps) % 10;
      this.score += this.position;
    }
    public clone(): Player {
      let ret = new Player(this.position, this.index);
      ret.score = this.score;
      ret.count = this.count;
      return ret;
    }
  }

  export function part1(desc: string, p1: Player, p2: Player) {
    let die: Die = new Die();
    let loop_count = 0;
    let currPlayer: Player;
    while (p1.score < 1000 && p2.score < 1000) {
      currPlayer = loop_count % 2 == 0 ? p1 : p2;
      currPlayer.move_by(die.roll_n_times(3));
      loop_count++;
    }
    let result = Math.min(p1.score, p2.score) * (loop_count * 3);
    console.log(`${desc}: ${p1.score} vs ${p2.score}: result=${result}`);
  }

  // In 3 rolls forking 3 times each, there are 27 (3^3) branches.
  // But, all 27 of those forks result in 9 different final numbers (with diffing probability)
  // 3	1			1
  // 4	2	1		3
  // 5	3	2	1	6
  // 6	2	3	2	7
  // 7	1	2	3	6
  // 8		1	2	3
  // 9			1	1

  function iterate_next_roll(current_player: Player, other_player: Player, win_count: number[]) {
    assert(current_player.score < 21);
    if (other_player.score >= 21) {
      win_count[other_player.index] += other_player.count * current_player.count;
      return;
    }
    for (let [roll, count] of QuantumDie.three_roll_options()) {
      let next_state = current_player.clone();
      next_state.move_by(roll);
      next_state.count *= count;
      iterate_next_roll(other_player, next_state, win_count);
    }
  }

  export function part2(desc: string, p1: Player, p2: Player) {
    let win_count: number[] = [0, 0];
    iterate_next_roll(p1, p2, win_count);
    console.log(`${desc}: Wins: ${win_count[0]} vs ${win_count[1]}`);
  }
}

// Test
// Player 1 starting position: 4
// Player 2 starting position: 8
// Real
// Player 1 starting position: 3
// Player 2 starting position: 4

export function solve() {
  console.log(`Day 20:`);
  d21.part1("P1 (test)", new d21.Player(4, 0), new d21.Player(8, 1));
  d21.part1("P1 (real)", new d21.Player(3, 0), new d21.Player(4, 1));
  d21.part2("P2 (test)", new d21.Player(4, 0), new d21.Player(8, 1));
  d21.part2("P2 (real)", new d21.Player(3, 0), new d21.Player(4, 1));
}

// P1 (test): 1000 vs 745: result=739785
// P1 (real): 912 vs 1001: result=995904
