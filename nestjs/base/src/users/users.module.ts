import { Module } from '@nestjs/common';
import { UsersController } from './users.controller';
import { TypeOrmModule } from '@nestjs/typeorm';
import { User } from './entities/user.entity';
import { UserGroup } from './entities/user-group.entity';
import { UsersResolver } from './graphql/users.resolver';
import { UsersService } from './users.service';

@Module({
  imports: [TypeOrmModule.forFeature([User, UserGroup])],
  controllers: [UsersController],
  providers: [UsersResolver, UsersService],
  exports: [UsersResolver],
})
export class UsersModule {}
