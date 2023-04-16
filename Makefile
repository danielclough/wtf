up: db_up
	docker compose up -d
down:
	docker compose down

db_up:
	docker compose -f 'docker-compose-db.yaml' up -d
db_down:
	docker compose -f 'docker-compose-db.yaml' down

# Start from nothing
init: db_up
	rustup override set nightly
	# init db
	# wait for docker
	sleep 10
	cd backend && bash sh/init.sh
	cd backend && cargo run -- --init

clean:
	docker container stop wtf_backend_postgres
	docker compose down
	docker system prune
	sudo rm -fr pg
	sudo rm -fr backend/migrations/*
	# sudo rm -fr backend/src/schema.rs

