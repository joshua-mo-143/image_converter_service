build:
	docker image build -t webpnator .

up:
	docker run -d -t -p 8333:8000 --name webpnator webpnator 

init:
	make build && make up

re:
	docker rm -f webpnator && make init
test: 
	curl -X POST -F file=@meme.tar.gz http://localhost:8000/convert --output result.tar.gz
