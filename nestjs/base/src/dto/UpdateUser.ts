import { IsEmail } from 'class-validator';
import { UserGroup } from '../entity/UserGroup';

export class UpdateUser {
  @IsEmail()
  readonly email: string;

  readonly groups: UserGroup[];
}
