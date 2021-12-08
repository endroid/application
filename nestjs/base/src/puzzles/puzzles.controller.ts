import { Controller, Get, Param } from '@nestjs/common';
import { SolversRegistry } from './solvers/solvers.registry';

@Controller('puzzles')
export class PuzzlesController {
  constructor(private solversRegistry: SolversRegistry) {}

  @Get('solve/:number')
  solve(@Param('number') number: number): number {
    return this.solversRegistry.get(number).solve();
  }
}
