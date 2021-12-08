import { ArgsType, Field, InputType, ObjectType } from '@nestjs/graphql';
import { UsersFilters } from '../../users/users.filters';
import { DateRangeFilters } from '../date-range.filters';

@InputType()
export class DateRangeArgs {
  @Field(() => Date, { nullable: true })
  from?: Date;

  @Field(() => Date, { nullable: true })
  to?: Date;

  toFilters(): any {
    const filters = new DateRangeFilters();
    filters.from = this.from;
    filters.to = this.to;
    return filters;
  }
}
