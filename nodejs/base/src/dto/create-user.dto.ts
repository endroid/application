import { IsEmail, IsString, IsUUID } from 'class-validator';
import { UserGroup } from '../entity/user-group.entity';

export class CreateUserDto {
  @IsUUID()
  readonly uuid: string;

  @IsEmail()
  readonly email: string;

  readonly groups: UserGroup[];
}
