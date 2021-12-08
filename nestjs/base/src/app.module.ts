import { AppController } from './app.controller';
import { TypeOrmModule } from '@nestjs/typeorm';
import { getConnectionOptions } from 'typeorm';
import { GraphQLModule } from '@nestjs/graphql';
import { Module } from '@nestjs/common';
import { UsersModule } from './users/users.module';
import { AppService } from './app.service';
import { PuzzlesModule } from './puzzles/puzzles.module';

@Module({
  imports: [
    AppModule,
    PuzzlesModule,
    UsersModule,
    GraphQLModule.forRootAsync({
      useFactory: () => ({
        include: [UsersModule],
        autoSchemaFile: true,
      }),
    }),
    TypeOrmModule.forRootAsync({
      useFactory: async () =>
        Object.assign(await getConnectionOptions(), {
          autoLoadEntities: true,
          logging: true,
        }),
    }),
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
