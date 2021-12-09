import { UserGroup } from './user-group.entity';
import {
  Column,
  CreateDateColumn,
  Entity,
  ManyToOne,
  PrimaryColumn,
  UpdateDateColumn,
} from 'typeorm';
import { Field, ObjectType } from '@nestjs/graphql';

@Entity()
@ObjectType()
export class User {
  @Field(() => String)
  @PrimaryColumn({ type: 'uuid' })
  readonly id: string;

  @Field(() => String)
  @Column({ unique: true })
  email: string;

  @Field(() => UserGroup)
  @ManyToOne(() => UserGroup)
  group: UserGroup;

  @Field(() => Date)
  @CreateDateColumn({ type: 'timestamptz' })
  readonly createdAt: Date;

  @Field(() => Date)
  @UpdateDateColumn({ type: 'timestamptz' })
  readonly updatedAt: Date;

  constructor(id: string, email: string, group: UserGroup) {
    this.id = id;
    this.email = email;
    this.group = group;
  }
}
