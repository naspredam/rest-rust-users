start-db:
	docker-compose up -d db

clean:
	docker image prune -f
	docker container prune -f

local-run:
	cargo run

start:
	docker build -t rest-rust-users .
	docker-compose up -d

stop:
	docker-compose down

logs:
	docker-compose logs -f

restart: clean stop start
