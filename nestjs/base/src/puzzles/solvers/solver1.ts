import { HttpService } from '@nestjs/axios';

export class Solver1 implements Solver {
  number = 1;

  private url = 'https://adventofcode.com/2021/day/1/input';

  constructor(private readonly httpService: HttpService) {}

  solve(): number {
    const content = this.httpService.get(this.url);

    console.log(content);

    return 1;
  }
}
