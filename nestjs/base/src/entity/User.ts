import { UserGroup } from './UserGroup';
import {Column, Entity, JoinTable, ManyToMany, PrimaryColumn} from 'typeorm';
import {Field, ObjectType} from "@nestjs/graphql";

@Entity()
@ObjectType()
export class User {
  @Field(type => String)
  @PrimaryColumn()
  readonly uuid: string;

  @Field()
  @Column()
  email: string;

  @JoinTable()
  @ManyToMany((type) => UserGroup, (group) => group.users)
  groups: UserGroup[];
}
