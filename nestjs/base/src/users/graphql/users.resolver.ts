import { User } from '../entities/user.entity';
import { Args, ArgsType, InputType, Query, Resolver } from '@nestjs/graphql';
import { UsersArgs } from './users.args';
import { UsersService } from '../users.service';
import { UsersFilters } from '../users.filters';

@Resolver(() => User)
export class UsersResolver {
  constructor(private readonly userService: UsersService) {}

  @Query(() => [User])
  async users(@Args() args: UsersArgs): Promise<User[]> {
    return this.userService.find(args.toFilters());
  }
}
