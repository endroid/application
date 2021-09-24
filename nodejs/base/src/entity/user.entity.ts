import { UserGroup } from './user-group.entity';
import {Column, Entity, JoinTable, ManyToMany, PrimaryColumn} from 'typeorm';

@Entity()
export class User {
  @PrimaryColumn()
  uuid: string;

  @Column()
  email: string;

  @JoinTable()
  @ManyToMany((type) => UserGroup, (group) => group.users)
  groups: UserGroup[];
}
