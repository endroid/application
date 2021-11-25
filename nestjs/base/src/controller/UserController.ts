import {
  Body,
  Controller,
  Delete,
  Get,
  NotFoundException,
  Param,
  Patch,
  Post,
} from '@nestjs/common';
import { CreateUser } from '../dto/CreateUser';
import { UpdateUser } from '../dto/UpdateUser';
import { InjectRepository } from '@nestjs/typeorm';
import { User } from '../entity/User';
import { Repository } from 'typeorm';

@Controller('user')
export class UserController {
  constructor(
    @InjectRepository(User)
    private readonly userRepository: Repository<User>,
  ) {}

  @Get()
  findAll() {
    return this.userRepository.find();
  }

  @Get(':uuid')
  async findOne(@Param('uuid') uuid: string) {
    const user = await this.userRepository.findOne(uuid);
    if (!user) {
      throw new NotFoundException(`User with UUID "${uuid}" not found`);
    }
    return user;
  }

  @Post()
  create(@Body() createUser: CreateUser) {
    const user = this.userRepository.create(createUser);
    return this.userRepository.save(user);
  }

  @Patch(':uuid')
  async update(
    @Param('uuid') uuid: string,
    @Body() updateUser: UpdateUser,
  ) {
    const user = await this.userRepository.preload({
      uuid: uuid,
      ...updateUser,
    });
    if (!user) {
      throw new NotFoundException(`User with UUID "${uuid}" not found`);
    }
    return this.userRepository.save(user);
  }

  @Delete(':uuid')
  async delete(@Param('uuid') uuid: string) {
    const user = await this.findOne(uuid);
    return this.userRepository.remove(user);
  }
}
