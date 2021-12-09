import { DateRangeFilter } from '../shared/date-range.filter';
import { ArgsType, Field, ID } from '@nestjs/graphql';
import { Type } from 'class-transformer';

@ArgsType()
export class UsersFilter {
  @Field(() => ID, { nullable: true })
  public id?: string;

  @Field(() => String, { nullable: true })
  public groupName?: string;

  @Type()
  @Field(() => DateRangeFilter, { nullable: true })
  public createdAt?: DateRangeFilter;

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
