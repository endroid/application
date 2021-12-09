"use strict";
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
var __param = (this && this.__param) || function (paramIndex, decorator) {
    return function (target, key) { decorator(target, key, paramIndex); }
};
exports.__esModule = true;
exports.PuzzlesController = void 0;
var common_1 = require("@nestjs/common");
var PuzzlesController = /** @class */ (function () {
    function PuzzlesController(solversRegistry) {
        this.solversRegistry = solversRegistry;
    }
    PuzzlesController.prototype.solve = function (number) {
        return this.solversRegistry.get(number).solve();
    };
    __decorate([
        (0, common_1.Get)('solve/:number'),
        __param(0, (0, common_1.Param)('number'))
    ], PuzzlesController.prototype, "solve");
    PuzzlesController = __decorate([
        (0, common_1.Controller)('puzzles')
    ], PuzzlesController);
    return PuzzlesController;
}());
exports.PuzzlesController = PuzzlesController;
