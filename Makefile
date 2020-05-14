DOCKER_IMAGE = purple:latest
run:
	docker run --rm -it ${DOCKER_IMAGE} cargo run
build:
	docker run --rm -it ${DOCKER_IMAGE} cargo build
test:
	docker run --rm -it ${DOCKER_IMAGE} cargo test
setup:
	docker build -t purple .
