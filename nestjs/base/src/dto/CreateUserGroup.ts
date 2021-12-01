import { IsString, IsUUID } from 'class-validator';
import { UserGroup } from '../entity/UserGroup';

export class CreateUserGroup {
  @IsUUID()
  readonly id: string;

  @IsString()
  readonly name: string;

  readonly groups: UserGroup[];
}
