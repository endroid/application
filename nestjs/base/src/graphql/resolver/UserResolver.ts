import {User} from "../../entity/User";
import {Args, Int, Query, Resolver} from "@nestjs/graphql";
import {InjectRepository} from "@nestjs/typeorm";
import {Repository} from "typeorm";

@Resolver()
export class UserResolver {
  constructor(
    @InjectRepository(User)
    private readonly userRepository: Repository<User>,
  ) {
  }

  @Query(returns => User)
  async author(@Args('id', { type: () => Int }) id: number) {
    return this.userRepository.findOne(id);
  }
}
