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

rebuild:
	docker image rm wtf-wtf_rocket:latest -f
	docker system prune
	make up

clean:
	docker container stop wtf_backend_postgres
	docker compose down
	docker system prune
	sudo rm -fr pg
	sudo rm -fr backend/migrations/*
	# sudo rm -fr backend/src/schema.rs

host:
	bash tools/sh/host.sh


whisper:
	bash tools/sh/whisper.sh