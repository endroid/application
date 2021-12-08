export class SolversRegistry {
  private solvers: Solver[] = [];

  public get(number: number): Solver {
    let solver = this.solvers.find((solver) => solver.number === number);

    if (!solver) {
      throw new Error('Solver with name "' + number + '" not found');
    }

    return solver;
  }

  public set(solver: Solver): void {
    this.solvers.push(solver);
  }
}
