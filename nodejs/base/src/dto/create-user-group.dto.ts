import { IsString, IsUUID } from 'class-validator';
import { UserGroup } from '../entity/user-group.entity';

export class CreateUserDto {
  @IsUUID()
  readonly uuid: string;

  @IsString()
  readonly name: string;

  readonly groups: UserGroup[];
}
