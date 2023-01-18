import { buffer } from "stream/consumers";
import DataHelpers from "./data/dataHelpers.js";

namespace d16 {
  class StringReader {
    constructor(buffer: string) {
      this.buff = buffer;
      this.pos = 0;
    }
    readAsString(length: number): string {
      let ret: string = this.buff.substring(this.pos, this.pos + length);
      this.pos += length;
      return ret;
    }
    readAsNumber(length: number): number {
      return +("0b" + this.readAsString(length));
    }
    get currentPos(): number {
      return this.pos;
    }
    private buff: string = "";
    private pos: number = 0;
  }

  function parsePacket(packetReader: StringReader, versionSum: { sum: number }) {
    let version: number = packetReader.readAsNumber(3);
    versionSum.sum += version;
    //console.log(`Version: +${version} = ${versionSum.sum}`);
    let ret: number = 0;
    let typeId: number = packetReader.readAsNumber(3);
    if (typeId == 4) {
      let more: boolean = true;
      let literalString: string = "0b";
      while (more) {
        more = packetReader.readAsNumber(1) > 0;
        literalString += packetReader.readAsString(4);
      }
      ret = +literalString;
      //console.log(`Literal = ${literal}`);
    } else {
      //console.log(`Op = ${typeId}`);
      let values: number[] = new Array<number>();
      let lengthTypeId: number = packetReader.readAsNumber(1);
      if (lengthTypeId == 0) {
        let bitCount: number = packetReader.readAsNumber(15);
        let packetEnd: number = packetReader.currentPos + bitCount;
        while (packetReader.currentPos < packetEnd) {
          values.push(parsePacket(packetReader, versionSum));
        }
      } else if (lengthTypeId == 1) {
        let packetCount: number = packetReader.readAsNumber(11);
        for (let i = 0; i < packetCount; i++) {
          values.push(parsePacket(packetReader, versionSum));
        }
      }
      switch (typeId) {
        case 0: // sum
          ret = values.reduce((p, c) => p + c, 0);
          break;
        case 1: //product
          ret = values.reduce((p, c) => p * c, 1);
          break;
        case 2: //min
          ret = Math.min(...values);
          break;
        case 3: //max
          ret = Math.max(...values);
          break;
        case 5: //greater
          ret = values[0] > values[1] ? 1 : 0;
          break;
        case 6: //less
          ret = values[0] < values[1] ? 1 : 0;
          break;
        case 7: //equal
          ret = values[0] == values[1] ? 1 : 0;
          break;
      }
    }
    return ret;
  }

  function parseAllPackets(packetsInHex: string[]) {
    var packets: string[] = packetsInHex.map((p) => {
      return Array.from(p).reduce((p, c) => p + ("0000" + (+("0x" + c)).toString(2)).slice(-4), "");
    });

    packets.forEach((p, i) => {
      console.log(packetsInHex[i] + ":");
      let versionSum: { sum: number } = { sum: 0 };
      let packetReader: StringReader = new StringReader(p);
      let result: number = parsePacket(packetReader, versionSum);
      console.log(`P1 Version Sum = ${versionSum.sum}`);
      console.log(`P2 Result = ${result}`);
    });
  }

  export function part1(desc: string, filename: string) {
    var packetsInHex: string[] = DataHelpers.fileToStringArray(filename);
    parseAllPackets(packetsInHex);
  }

  export function part2(desc: string, filename: string) {
    var packetsInHex: string[] = DataHelpers.fileToStringArray(filename);
    parseAllPackets(packetsInHex);
  }
}

export function solve() {
  d16.part1("test", "./data/d16_test.txt");
  d16.part1("real", "./data/d16_real.txt");
  d16.part2("test", "./data/d162_test.txt");
  d16.part2("real", "./data/d16_real.txt");
}
