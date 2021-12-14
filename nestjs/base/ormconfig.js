require("dotenv").config({ path: __dirname + `/.env.${process.env.NODE_ENV}`});

module.exports = {
	type: "postgres",
	host: "postgres",
	port: 5432,
	username: "root",
	password: "root",
	database: process.env.DATABASE_NAME,
	synchronize: false,
	logging: process.env.NODE_ENV !== 'production',
	entities: ["dist/**/*.entity.js"],
	migrations: ["dist/migrations/*.js"],
	cli: {
		migrationsDir: "src/migrations"
	}
}
