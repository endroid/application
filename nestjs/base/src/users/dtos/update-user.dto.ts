import { IsEmail } from 'class-validator';
import { UserGroup } from '../entities/user-group.entity';

export class UpdateUser {
  @IsEmail()
  readonly email: string;

  readonly groups: UserGroup[];
}
