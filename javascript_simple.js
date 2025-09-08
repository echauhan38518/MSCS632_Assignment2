function outer(x) {
    return function inner(y) {
        return x + y;
    };
}

let addFive = outer(5);
console.log(addFive(3)); 