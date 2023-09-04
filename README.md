# Imageconverter: A self-hostable service for converting images to WebP format.
Do you also hate manually converting images to webp format one at a time? Me too! I created this service which leverages the high power of Rust to be able to efficiently process images in bulk. This simple service has only one endpoint at `/convert`, which takes either a single image or a compressed tarball (`.tar.gz`) format. It will then either return a single image or compressed tarball depending on what you send to the endpoint. Now you can do it yourself. 

As a warning: If you're trying to convert large amounts of images (100GB+) at once, I would highly suggest chunking it in blocks, especially if you have bad internet speed. 

## How to Run
You can run this service locally by cloning the repo and using `cargo run`. 

You can also build the Docker image. It exposes port 8000 by default.

Once you've built the Docker image and it's running (or you're running the service locally), you can try sending the service an image or compressed tarball!

You can then go to [http://localhost:8000](http://localhost:8000) (or whatever port you're running the service on) to access the web form!

You can also use curl to send a request, like so:

```bash
curl -X POST -F file=@<image-or-archive-file> http://localhost:8000/convert
```
## TODO
- Make the web form less ugly
- Support more image types
