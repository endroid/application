import { Field, ArgsType, InputType, ObjectType, ID } from '@nestjs/graphql';
import { DateRangeArgs } from '../../shared/graphql/date-range.args';
import { UsersFilters } from '../users.filters';
import { Type } from 'class-transformer';

@ArgsType()
export class UsersArgs {
  @Field(() => ID, { nullable: true })
  id?: string;

  @Field(() => String, { nullable: true })
  groupName?: string;

  @Type()
  @Field(() => DateRangeArgs, { nullable: true })
  createdAt?: DateRangeArgs;

  toFilters(): UsersFilters {
    const filters = new UsersFilters();
    filters.id = this.id;
    filters.groupName = this.groupName;
    filters.createdAt = this.createdAt?.toFilters();
    return filters;
  }
}
