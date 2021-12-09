import { User } from '../entities/user.entity';
import { Args, Query, Resolver } from '@nestjs/graphql';
import { UsersService } from '../users.service';
import { UsersFilter } from '../users.filter';

@Resolver(() => User)
export class UsersResolver {
  constructor(private readonly userService: UsersService) {}

  @Query(() => [User])
  async users(@Args() filter: UsersFilter): Promise<User[]> {
    return this.userService.find(filter);
  }
}
