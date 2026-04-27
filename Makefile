dev:
	cargo watch -x run

run:
	cargo run

health:
	curl http://localhost:3000/health

predictions:
	curl http://localhost:3000/predictions?product_id=$(product_id)

postgres-up:
	docker run -d -- name postgres-14 -p 5432:5432 -e POSTGRES_PASSWORD=123456 postgres:14

postgres-down:
	docker stop postgres-14
	docker rm postgres-14