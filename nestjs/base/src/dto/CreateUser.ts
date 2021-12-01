import { IsEmail, IsString, IsUUID } from 'class-validator';
import { UserGroup } from '../entity/UserGroup';

export class CreateUser {
  @IsUUID()
  readonly id: string;

  @IsEmail()
  readonly email: string;

  readonly groups: UserGroup[];
}
