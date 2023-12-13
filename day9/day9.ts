let message: string = "Advent of code ~~~~~ Day9";
console.log(message);

// Part 1
let fs = require("fs");
let text = fs.readFileSync("./input.txt", "utf-8");
let data = text.split("\n");

function get_next_value(serie) {
    if (serie.every(n => n == 0)) {
        return 0;
    } else {
        let diff = new Array(serie.length - 1);
        for (let i = 0; i < serie.length - 1; i++) {
            diff[i] = serie[i + 1] - serie[i];
        }
        return serie[serie.length - 1] + get_next_value(diff);
    }
}

function get_previous_value(serie) {
    if (serie.every(n => n == 0)) {
        return 0;
    } else {
        let diff = new Array(serie.length - 1);
        for (let i = 0; i < serie.length - 1; i++) {
            diff[i] = serie[i + 1] - serie[i];
        }
        return serie[0] - get_previous_value(diff);
    }

}

let part1 = 0;
let part2 = 0;
data.forEach(line => {
    let numbers_string = line.split(" ");
    let numbers = numbers_string.map(Number);
    part1 += get_next_value(numbers);
    part2 += get_previous_value(numbers);
});
console.log("Part 1: " + part1);
console.log("Part 2: " + part2);
