# WebPNator: A self-hostable service for converting images to WebP format.
Do you also hate manually converting images to webp format one at a time? Me too! I created this service which leverages the high power of Rust to be able to efficiently process images in bulk. This simple service has a webpage form at `/` where you can manually submit your images to be processed, as well as a `/convert` endpoint for doing it manually (currently only takes a single file upload). It will then either return a single image or compressed tarball depending on what you send to the endpoint. Now you can leverage the power of Rust for yourself, without any of the work! 

As a warning: If you're trying to convert large amounts of images (100GB+) at once, I would highly suggest chunking it in blocks, especially if you have bad internet speed. 

## Quickstart
Pull the image and run it:
```bash
docker pull smtmsblst/webpnator && docker run -d -t -p 8333:8000 --name webpnator smtmsblst/webpnator
```

Now all you have to do is visit [http://localhost:8333](http://localhost:8333) and it'll bring up the web form! You can also of course change the port if you want - the web service itself however runs at port 8000.

You can also use curl to send a request, like so:

```bash
curl -X POST -F file=@<image-or-archive-file> http://localhost:8000/convert
```

## Testing/Development
You can run this service locally by cloning the repo and using `cargo run`. 

The ideal way to test this app is to use `make init` (assuming you have CMake installed) to get the app image loaded up. You can then use `make re` to reinitialise the image. 

You can also build the Docker image manually by using `docker image build -t webpnator .`.

If you're running locally without the image, the web service will be found at port 8000 compared to 8333 in the quickstart.

Once you've built the Docker image and it's running (or you're running the service locally), you can try sending the service an image or compressed tarball!


## TODO
- Make the web form less ugly
- Support more image types
- Nix docker build support
