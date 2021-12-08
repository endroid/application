import { Module } from '@nestjs/common';
import { HttpModule } from '@nestjs/axios';
import { PuzzlesController } from './puzzles.controller';
import { SolversRegistry } from './solvers/solvers.registry';

@Module({
  imports: [
    HttpModule.registerAsync({
      useFactory: () => ({
        timeout: 5000,
        maxRedirects: 5,
      }),
    }),
  ],
  controllers: [PuzzlesController],
  providers: [SolversRegistry],
  exports: [],
})
export class PuzzlesModule {}
