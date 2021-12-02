import { Module } from '@nestjs/common';
import { UsersController } from './users.controller';
import { TypeOrmModule } from '@nestjs/typeorm';
import { User } from './entities/user.entity';
import { UserGroup } from './entities/user-group.entity';
import { UsersResolver } from './resolvers/users.resolver';

@Module({
  imports: [TypeOrmModule.forFeature([User, UserGroup])],
  controllers: [UsersController],
  providers: [UsersResolver],
  exports: [UsersResolver],
})
export class UsersModule {}
