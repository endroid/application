import { User } from '../../entity/User';
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
import { UserGroup } from '../../entity/UserGroup';

@Resolver(() => User)
export class UserResolver {
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
