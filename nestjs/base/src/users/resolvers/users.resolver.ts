import { User } from '../entities/user.entity';
import {
  Args,
  ArgsType,
  Int,
  Parent,
  Query,
  ResolveField,
  Resolver,
} from '@nestjs/graphql';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { UserGroup } from '../entities/user-group.entity';

@Resolver(() => User)
export class UsersResolver {
  constructor(
    @InjectRepository(User)
    private readonly userRepository: Repository<User>,
    @InjectRepository(UserGroup)
    private readonly userGroupRepository: Repository<UserGroup>,
  ) {}

  @Query(() => User)
  async user(@Args('id', { type: () => String }) id: string) {
    return this.userRepository.findOne(
      { id: id },
      {
        relations: ['groups'],
      },
    );
  }
}
