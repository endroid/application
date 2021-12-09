export class SolversRegistry {
  private solvers: Solver[] = [];

  get(number: number): Solver {
    const solver = this.solvers.find((solver) => solver.number === number);

    if (!solver) {
      throw new Error('Solver with name "' + number + '" not found');
    }

    return solver;
  }

  set(solver: Solver): void {
    this.solvers.push(solver);
  }
}
