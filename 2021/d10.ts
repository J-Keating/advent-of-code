import d10Data from "./data/d10_data.js";

namespace d10 {
  const scoreMap: Map<string, number> = new Map<string, number>([
    [")", 3],
    ["]", 57],
    ["}", 1197],
    [">", 25137],
  ]);
  const matchMap: Map<string, string> = new Map<string, string>([
    ["(", ")"],
    ["[", "]"],
    ["{", "}"],
    ["<", ">"],
  ]);

  export function part1(desc: string, lines: string[]): string[] {
    let score: number = 0;
    let completeStrings: Array<string> = new Array<string>();

    lines.forEach((line) => {
      let stack: Array<string> = new Array<string>();
      var curChar: string = "";
      var errorChar: string = "";
      while (line.length > 0 && errorChar.length == 0) {
        [curChar, line] = [line.charAt(0), line.substring(1)];
        if (matchMap.has(curChar)) {
          stack.push(matchMap.get(curChar));
        } else if ([...matchMap.values()].includes(curChar)) {
          let wantChar: string = stack.pop();
          if (wantChar != curChar) {
            //console.log(`Expected ${wantChar}, but found ${curChar} instead`);
            errorChar = curChar;
          }
        } else {
          console.log(`Mismatch!  cur=${curChar}  Stack=${stack}  line=${line}`);
        }
      }

      if (errorChar.length > 0) {
        score += scoreMap.get(errorChar);
      } else {
        let completeString: string = stack.reduce((p, n) => n + p);
        //console.log(`Incomplete line.  Complete Chars = ${completeString}`);
        completeStrings.push(completeString);
      }
    });

    if (desc.length > 0) {
      console.log(`P1 Total (${desc}) = ${score}`);
    }
    return completeStrings;
  }

  const endScoreMap: Map<string, number> = new Map<string, number>([
    [")", 1],
    ["]", 2],
    ["}", 3],
    [">", 4],
  ]);

  export function part2(desc: string, lines: string[]) {
    let completeStrings: String[] = part1("", lines);
    let scores: Array<number> = completeStrings.map((s) => {
      return [...s].reduce((p, c) => p * 5 + endScoreMap.get(c), 0);
    });
    let ret: number = scores.sort((a, b) => a - b)[(scores.length - 1) / 2];
    console.log(`P2 Score (${desc}) = ${ret}`);
  }
}

// P1 Total (test) = 26397
// P1 Total (real) = 436497
// P2 Score (test) = 288957
// P2 Score (real) = 2377613374

d10.part1("test", d10Data.testData);
d10.part1("real", d10Data.realData);
d10.part2("test", d10Data.testData);
d10.part2("real", d10Data.realData);
