import { DateRangeFilter } from '../shared/date-range.filter';
import { ArgsType, Field } from '@nestjs/graphql';
import { Type } from 'class-transformer';

@ArgsType()
export class UsersFilter {
  @Field(() => String, { nullable: true })
  id?: string;

  @Field(() => String, { nullable: true })
  groupName?: string;

  @Type()
  @Field(() => DateRangeFilter, { nullable: true })
  createdAt?: DateRangeFilter;

  toOrmFilter(ormFilter: any = {}): any {
    if (this.id) {
      ormFilter.id = this.id;
    }
    if (this.groupName) {
      ormFilter.group = { name: this.groupName };
    }
    if (this.createdAt) {
      this.createdAt.toOrmFilter(ormFilter, 'createdAt');
    }
    return ormFilter;
  }
}
