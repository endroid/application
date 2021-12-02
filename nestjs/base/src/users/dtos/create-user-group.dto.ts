import { IsString, IsUUID } from 'class-validator';
import { UserGroup } from '../entities/user-group.entity';

export class CreateUserGroup {
  @IsUUID()
  readonly id: string;

  @IsString()
  readonly name: string;

  readonly groups: UserGroup[];
}
