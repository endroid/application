import {Column, Entity, ManyToMany, PrimaryColumn} from "typeorm";
import {User} from "./User";

@Entity()
export class UserGroup {
  @PrimaryColumn()
  readonly uuid: string;

  @Column()
  name: string;

  @ManyToMany(
      type => User,
      user => user.groups,
  )
  users: User[];
}
