default: run

RUNINGS := $(docker ps -a -q --filter ancestor=coc-autoskip)

build:
	docker build -t coc-autoskip .

check: build
	docker run -t coc-autoskip \
		cargo test --release

run:
	docker run -t coc-autoskip

kill:
	docker kill $(RUNINGS)
	docker rm $(RUNINGS)

clean:
	cargo clean
