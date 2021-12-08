import { DateRangeFilters } from '../shared/date-range.filters';

export class UsersFilters {
  id?: string;
  groupName?: string;
  createdAt?: DateRangeFilters;

  toOrmFilters(ormFilters: any = {}): any {
    if (this.id) {
      ormFilters.id = this.id;
    }
    if (this.groupName) {
      ormFilters.group = { name: this.groupName };
    }
    if (this.createdAt) {
      this.createdAt.toOrmFilters(ormFilters, 'createdAt');
    }
    return ormFilters;
  }
}
