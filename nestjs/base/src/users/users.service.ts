import { User } from './entities/user.entity';
import { InjectRepository } from '@nestjs/typeorm';
import { Between, Repository } from 'typeorm';
import { UsersArgs } from './graphql/users.args';
import { UsersFilters } from './users.filters';

export class UsersService {
  constructor(
    @InjectRepository(User)
    private userRepository: Repository<User>,
  ) {}

  async find(filters: UsersFilters): Promise<User[]> {
    return await this.userRepository.find({
      where: filters.toOrmFilters(),
      relations: ['group'],
    });
  }
}
