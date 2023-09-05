<!DOCTYPE HTML>
<head>
<title>WebPNator</title>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<main>
<h1>Webpnator</h1> 
	
<form method="POST" action="/convert" enctype="multipart/form-data">
<label for="file">

## Upload a file:

<input type="file" name="file"></input>
</label>
<button type="submit">Upload</button>	
</form>

## How to Use
All you need to do is upload an image and it'll send it back to you as a .webp file. No ads, no mess. 

Currently, JPEGs, JPGs, PNGs (and compressed tarballs) are accepted. More will be coming in the future.
</main>

<style>
main {
width: 60%;
}

main {
display: flex;
width: 100vw;
flex-direction: column;
align-items: center;
justify-content: center;
text-align: center;
}

form {
padding: 2rem 1rem;
background-color: rgb(235, 216, 243);
}
</style>

