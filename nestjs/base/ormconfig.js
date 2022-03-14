require("dotenv").config({ path: __dirname + `/.env.${process.env.NODE_ENV}`});

module.exports = {
	type: "postgres",
	url: process.env.DATABASE_URL,
	synchronize: false,
	logging: process.env.NODE_ENV !== 'production',
	entities: ["dist/**/*.entity.js"],
	migrations: ["dist/migrations/*.js"],
	cli: {
		migrationsDir: "src/migrations"
	}
}
