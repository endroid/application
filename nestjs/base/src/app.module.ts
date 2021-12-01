import { Module } from '@nestjs/common';
import { AppController } from './controller/AppController';
import { UserController } from './controller/UserController';
import { TypeOrmModule } from '@nestjs/typeorm';
import { User } from './entity/User';
import { UserGroup } from './entity/UserGroup';
import { getConnectionOptions } from 'typeorm';
import { GraphQLModule } from '@nestjs/graphql';
import { UserResolver } from './graphql/resolver/UserResolver';

@Module({
  imports: [
    AppModule,
    GraphQLModule.forRootAsync({
      useFactory: () => ({
        autoSchemaFile: true,
      }),
    }),
    TypeOrmModule.forRootAsync({
      useFactory: async () =>
        Object.assign(await getConnectionOptions(), {
          autoLoadEntities: true,
        }),
    }),
    TypeOrmModule.forFeature([User, UserGroup]),
  ],
  controllers: [AppController, UserController],
  providers: [UserResolver],
})
export class AppModule {}
