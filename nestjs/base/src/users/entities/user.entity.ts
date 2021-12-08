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
  @PrimaryColumn()
  readonly id: string;

  @Field(() => String)
  @Column()
  email: string;

  @Field(() => UserGroup)
  @ManyToOne(() => UserGroup, (group) => group.users)
  group: UserGroup;

  @Field(() => Date)
  @CreateDateColumn({ type: 'timestamptz' })
  createdAt: Date;

  @Field(() => Date)
  @UpdateDateColumn({ type: 'timestamptz' })
  updatedAt: Date;
}
