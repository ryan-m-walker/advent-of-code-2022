const fs = require("fs")

const input = fs
    .readFileSync("./input.txt", "utf-8")
    .split("\n")
    .map((line) => line.split(/[-,]/))
    .flat()
    .reduce((max, x) => {
        const num = Number(x)
        if (num > max) {
            max = num
        }
        return max
    }, 0)

console.log(input)
