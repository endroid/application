import { Column, Entity, ManyToMany, PrimaryColumn } from 'typeorm';
import { User } from './user.entity';
import { Field, ObjectType } from '@nestjs/graphql';

@Entity()
@ObjectType()
export class UserGroup {
  @Field(() => String)
  @PrimaryColumn()
  readonly id: string;

  @Field()
  @Column()
  name: string;

  @ManyToMany(() => User, (user) => user.groups)
  users: User[];
}
