import { Field, InputType } from '@nestjs/graphql';
import { Between, LessThan, MoreThan } from 'typeorm';

@InputType()
export class DateRangeFilter {
  @Field(() => Date, { nullable: true })
  public from?: Date;

  @Field(() => Date, { nullable: true })
  public to?: Date;

  toOrmFilter(ormFilter: any = {}, name: string): any {
    if (this.from instanceof Date && this.to instanceof Date) {
      ormFilter[name] = Between(this.from, this.to);
    } else if (this.from instanceof Date) {
      ormFilter[name] = MoreThan(this.from);
    } else if (this.to instanceof Date) {
      ormFilter[name] = LessThan(this.to);
    }
  }
}
