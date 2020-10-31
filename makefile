start-db:
	docker-compose up -d db

local-run:
	cargo run

start:
	docker build -t rest-rust-users .
	docker-compose up -d

stop:
	docker-compose down

restart:
	make stop && make start

logs:
	docker-compose logs -f