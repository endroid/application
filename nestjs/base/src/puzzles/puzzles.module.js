"use strict";
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
exports.__esModule = true;
exports.PuzzlesModule = void 0;
var common_1 = require("@nestjs/common");
var axios_1 = require("@nestjs/axios");
var puzzles_controller_1 = require("./puzzles.controller");
var solvers_registry_1 = require("./solvers/solvers.registry");
var PuzzlesModule = /** @class */ (function () {
    function PuzzlesModule() {
    }
    PuzzlesModule = __decorate([
        (0, common_1.Module)({
            imports: [
                axios_1.HttpModule.registerAsync({
                    useFactory: function () { return ({
                        timeout: 5000,
                        maxRedirects: 5
                    }); }
                }),
            ],
            controllers: [puzzles_controller_1.PuzzlesController],
            providers: [solvers_registry_1.SolversRegistry],
            exports: []
        })
    ], PuzzlesModule);
    return PuzzlesModule;
}());
exports.PuzzlesModule = PuzzlesModule;
