import { IsEmail } from 'class-validator';
import { UserGroup } from '../entity/user-group.entity';

export class UpdateUserDto {
  @IsEmail()
  readonly email: string;

  readonly groups: UserGroup[];
}
