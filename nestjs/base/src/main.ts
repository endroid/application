import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { ValidationPipe } from '@nestjs/common';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  app.useGlobalPipes(
    new ValidationPipe({
      transform: true,
      forbidNonWhitelisted: true,
    }),
  );

  const hostname = '0.0.0.0';
  const port = 3000;

  await app.listen(port, hostname, () => {
    console.log(`Server running at http://${hostname}:${port}/`);
  });
}
bootstrap();
