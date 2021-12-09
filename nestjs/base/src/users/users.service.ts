import { User } from './entities/user.entity';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { UsersFilter } from './users.filter';

export class UsersService {
  constructor(
    @InjectRepository(User)
    private userRepository: Repository<User>,
  ) {}

  async find(filter: UsersFilter): Promise<User[]> {
    return await this.userRepository.find({
      where: filter.toOrmFilter(),
      relations: ['group'],
    });
  }
}
