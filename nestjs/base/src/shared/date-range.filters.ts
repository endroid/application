import { Between, LessThan, MoreThan } from 'typeorm';

export class DateRangeFilters {
  from?: Date;
  to?: Date;

  toOrmFilters(ormFilters: any = {}, name: string): any {
    if (this.from instanceof Date && this.to instanceof Date) {
      ormFilters[name] = Between(this.from, this.to);
    }
    if (this.from instanceof Date) {
      ormFilters[name] = MoreThan(this.from);
    }
    if (this.to instanceof Date) {
      ormFilters[name] = LessThan(this.to);
    }
  }
}
