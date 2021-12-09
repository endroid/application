"use strict";
exports.__esModule = true;
exports.Solver1 = void 0;
var Solver1 = /** @class */ (function () {
    function Solver1(httpService) {
        this.httpService = httpService;
        this.number = 1;
        this.url = 'https://adventofcode.com/2021/day/1/input';
    }
    Solver1.prototype.solve = function () {
        var content = this.httpService.get(this.url);
        console.log(content);
        return 1;
    };
    return Solver1;
}());
exports.Solver1 = Solver1;
