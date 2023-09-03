test: 
	curl -X POST -F file=@meme.tar.gz http://localhost:8000/convert --output result.tar.gz
