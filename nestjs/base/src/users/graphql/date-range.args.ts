import {Field,  InputType} from '@nestjs/graphql';

@InputType()
export class DateRangeArgs {
  @Field(() => Date,{ nullable: true })
  from?: Date;

  @Field(() => Date,{ nullable: true })
  to?: Date;
}
