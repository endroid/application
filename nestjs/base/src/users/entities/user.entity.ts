import { UserGroup } from './user-group.entity';
import { Column, Entity, JoinTable, ManyToMany, PrimaryColumn } from 'typeorm';
import { Field, ObjectType } from '@nestjs/graphql';

@Entity()
@ObjectType()
export class User {
  @Field((type) => String)
  @PrimaryColumn()
  readonly id: string;

  @Field(() => String)
  @Column()
  email: string;

  @Field(() => [UserGroup])
  @JoinTable()
  @ManyToMany((type) => UserGroup, (group) => group.users)
  groups: UserGroup[];
}
