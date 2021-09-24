import { Module } from '@nestjs/common';
import { AppController } from './controller/app.controller';
import { UserController } from './controller/user.controller';
import { TypeOrmModule } from '@nestjs/typeorm';
import { User } from './entity/user.entity';
import { UserGroup } from './entity/user-group.entity';
import { getConnectionOptions } from 'typeorm';

@Module({
  imports: [
    TypeOrmModule.forRootAsync({
      useFactory: async () =>
        Object.assign(await getConnectionOptions(), {
          autoLoadEntities: true,
        }),
    }),
    TypeOrmModule.forFeature([User, UserGroup]),
  ],
  controllers: [AppController, UserController],
  providers: [],
})
export class AppModule {}
