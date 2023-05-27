# List available recipes
help:
	@just --list

# Creates the container
build:
	docker-compose build

# Creates the container if necessary and starts it
up:
	docker-compose up -d

# Stops the container
down:
	docker-compose down

# Restarts the container
restart:
	@just down
	@just up
