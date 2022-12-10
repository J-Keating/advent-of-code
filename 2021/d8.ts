import d8Data from "./data/d8_data.js";

namespace d8 {
  export function part1(desc: string, rawDataString: string) {
    let lines: string[] = rawDataString.split("\n");
    let count: number = 0;
    lines.forEach((line: string) => {
      let splitline: string[] = line.split(`|`);
      let digits: string[] = splitline[1].trim().split(" ");
      count += digits.filter((x) => x.length == 2 || x.length == 3 || x.length == 4 || x.length == 7).length;
    });
    console.log(`Total (${desc}) = ${count}`);
  }

  function stringIntersection(s1: string, s2: string): number {
    let count: number = 0;
    for (let c1 of s1) {
      if (s2.includes(c1)) {
        count++;
      }
    }
    return count;
  }

  export function part2(desc: string, rawDataString: string) {
    // length == 2 --> "1"
    // length == 3 --> "7"
    // length == 4 --> "4"
    // length == 7 --> "8"
    // length == 6 && intersection with "1" == 1 --> "6"
    // length == 5 && intersection with "1" == 2 --> "3"
    // length == 5 && intersection with "6" == 5 --> "5"
    // length == 5 && !assigned --> "2"
    // length == 6 && intersection with "3" == 5 --> "9"
    // length == 6 && !assigned --> "0"

    let lines: string[] = rawDataString.split("\n");
    let count: number = 0;
    lines.forEach((line: string) => {
      let numberCodes: string[] = new Array<string>(10).fill("");
      let splitline: string[] = line.split(`|`);
      let inCodes: string[] = splitline[0].trim().split(" ");
      let encodedDigits: string[] = splitline[1].trim().split(" ");

      numberCodes[1] = inCodes.filter((c) => c.length == 2)[0];
      numberCodes[7] = inCodes.filter((c) => c.length == 3)[0];
      numberCodes[4] = inCodes.filter((c) => c.length == 4)[0];
      numberCodes[8] = inCodes.filter((c) => c.length == 7)[0];
      numberCodes[6] = inCodes.filter((c) => c.length == 6 && stringIntersection(c, numberCodes[1]) == 1)[0];
      numberCodes[3] = inCodes.filter((c) => c.length == 5 && stringIntersection(c, numberCodes[1]) == 2)[0];
      numberCodes[5] = inCodes.filter((c) => c.length == 5 && stringIntersection(c, numberCodes[6]) == 5)[0];
      numberCodes[2] = inCodes.filter((c, i) => c.length == 5 && !numberCodes.includes(c))[0];
      numberCodes[9] = inCodes.filter((c) => c.length == 6 && stringIntersection(c, numberCodes[3]) == 5)[0];
      numberCodes[0] = inCodes.filter((c, i) => c.length == 6 && !numberCodes.includes(c))[0];

      let outDigits:number[] = encodedDigits.map(enc => numberCodes.findIndex(c => c.length == enc.length && stringIntersection(c, enc) == c.length))
      count += outDigits.reduce((p, c) => 10*p + c, 0);
    });  

    console.log(`Total (${desc}) = ${count}`);
  }
}

//d8.part1("test", d8Data.testData);
//d8.part1("real", d8Data.realData);
d8.part2("test", d8Data.testData);
d8.part2("real", d8Data.realData);
