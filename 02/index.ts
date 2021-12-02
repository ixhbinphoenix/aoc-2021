import { readFileSync } from "fs";

let x: number = 0;
let depth: number = 0;
let aim: number = 0;

enum instructionType {
    forward = "forward",
    down = "down",
    up = "up"
}

class instruction {
    constructor(type: keyof typeof instructionType, val: number) {
        this.type = type;
        this.val = val;
    }
    type: keyof typeof instructionType;
    val: number
}

let instStr: string[] = readFileSync("input.txt").toString().split("\n");

function stringToInstruction(inp: string): instruction {
    let inparr = inp.split(" ");
    let inst = inparr[0] as keyof typeof instructionType;
    return new instruction(inst, Number(inparr[1]));
}

let instructions: instruction[] = [];
for (let i = 0; i < instStr.length; i++) {
    instructions[i] = stringToInstruction(instStr[i]);
}

function execInstruction(inst: instruction) {
    switch (inst.type) {
        case instructionType.forward:
            x += inst.val;
            depth += aim * inst.val;
            break;
        case instructionType.down:
            aim += inst.val;
            break;
        case instructionType.up:
            aim -= inst.val;
            break;
    }
}
instructions.forEach((inst) => {
    execInstruction(inst);
})

console.log(x * depth);