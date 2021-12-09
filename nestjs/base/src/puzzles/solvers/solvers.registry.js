"use strict";
exports.__esModule = true;
exports.SolversRegistry = void 0;
var SolversRegistry = /** @class */ (function () {
    function SolversRegistry() {
        this.solvers = [];
    }
    SolversRegistry.prototype.get = function (number) {
        var solver = this.solvers.find(function (solver) { return solver.number === number; });
        if (!solver) {
            throw new Error('Solver with name "' + number + '" not found');
        }
        return solver;
    };
    SolversRegistry.prototype.set = function (solver) {
        this.solvers.push(solver);
    };
    return SolversRegistry;
}());
exports.SolversRegistry = SolversRegistry;
