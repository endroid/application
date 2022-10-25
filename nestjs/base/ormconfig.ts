import { DataSource } from 'typeorm';
import * as dotenv from 'dotenv';
import { PostgresConnectionOptions } from 'typeorm/driver/postgres/PostgresConnectionOptions';

dotenv.config({
  path: __dirname + `/.env.${process.env.NODE_ENV}`
});

const configs: PostgresConnectionOptions = {
  type: 'postgres',
  synchronize: false,
  logging: process.env.NODE_ENV !== 'production',
  url: process.env.DATABASE_URL,
  entities: ['src/**/*.entity.ts'],
  migrations: ['src/migrations/*.ts']
};

export default new DataSource(configs);
