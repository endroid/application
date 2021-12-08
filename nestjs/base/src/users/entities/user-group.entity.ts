import { Column, Entity, ManyToMany, OneToMany, PrimaryColumn } from 'typeorm';
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

  @OneToMany(() => User, (user) => user.group)
  users: User[];
}
