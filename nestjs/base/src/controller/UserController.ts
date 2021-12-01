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

  @Get(':id')
  async findOne(@Param('id') id: string) {
    const user = await this.userRepository.findOne(id);
    if (!user) {
      throw new NotFoundException(`User with UUID "${id}" not found`);
    }
    return user;
  }

  @Post()
  create(@Body() createUser: CreateUser) {
    const user = this.userRepository.create(createUser);
    return this.userRepository.save(user);
  }

  @Patch(':id')
  async update(@Param('id') id: string, @Body() updateUser: UpdateUser) {
    const user = await this.userRepository.preload({
      id: id,
      ...updateUser,
    });
    if (!user) {
      throw new NotFoundException(`User with UUID "${id}" not found`);
    }
    return this.userRepository.save(user);
  }

  @Delete(':id')
  async delete(@Param('id') id: string) {
    const user = await this.findOne(id);
    return this.userRepository.remove(user);
  }
}
