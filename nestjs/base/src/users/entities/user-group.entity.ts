import { Column, Entity, ManyToMany, OneToMany, PrimaryColumn } from 'typeorm';
import { User } from './user.entity';
import { Field, ObjectType } from '@nestjs/graphql';

@Entity()
@ObjectType()
export class UserGroup {
  @Field(() => String)
  @PrimaryColumn({ type: 'uuid' })
  readonly id: string;

  @Field()
  @Column()
  name: string;

  constructor(id: string, name: string) {
    this.id = id;
    this.name = name;
  }
}
