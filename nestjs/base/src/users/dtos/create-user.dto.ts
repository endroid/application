import { IsEmail, IsString, IsUUID } from 'class-validator';
import { UserGroup } from '../entities/user-group.entity';

export class CreateUser {
  @IsUUID()
  readonly id: string;

  @IsEmail()
  readonly email: string;

  readonly groups: UserGroup[];
}
