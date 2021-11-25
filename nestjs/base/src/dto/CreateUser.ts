import { IsEmail, IsString, IsUUID } from 'class-validator';
import { UserGroup } from '../entity/UserGroup';

export class CreateUser {
  @IsUUID()
  readonly uuid: string;

  @IsEmail()
  readonly email: string;

  readonly groups: UserGroup[];
}
