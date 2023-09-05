use async_compression::futures::bufread::GzipDecoder;
use async_compression::tokio::write::GzipEncoder;
use async_std::stream::StreamExt;
use async_tar::Header;
use async_tar::{Archive, Builder};
use async_zip::tokio::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use axum::{
    body::Bytes,
    extract::Multipart,
    http::{
        header::{self, HeaderMap},
        StatusCode,
    },
    response::IntoResponse,
};
use image::io::Reader;
use image::ImageFormat;
use rand::distributions::{Alphanumeric, DistString};
use std::io::Cursor;
use std::path::PathBuf;
use tokio::fs::{create_dir, read_dir, File};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

pub async fn convert_image(multipart: Multipart) -> Result<impl IntoResponse, StatusCode> {
    let (filename, filedata) = handle_multipart(multipart).await.unwrap();

    let header_map = if filename.clone().unwrap().as_str().ends_with("tar.gz") {
        create_header_map(filename.clone().unwrap(), ContentType::Gzip)
    } else {
        create_header_map(filename.clone().unwrap(), ContentType::WebP)
    };

    Ok((StatusCode::OK, header_map, filedata.unwrap()))
}

pub async fn handle_multipart(
    mut multipart: Multipart,
) -> Result<(Option<String>, Option<Vec<u8>>), String> {
    let mut filedata: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        filename = if filename.is_none() {
            Some(field.file_name().unwrap().to_owned())
        } else {
            return Err("You can only have one file upload at a time! Upload .zip files or .tar.gz if you want multiple files to be done at once.".to_string());
        };

        let bytes = field.bytes().await.unwrap();

        let meme = filename.clone().unwrap();

        let filepath = std::path::Path::new(&meme);

        filedata = match filepath.extension() {
            None => None,
            Some(os_str) => match os_str.to_str() {
                Some("webp") | Some("png") | Some("jpeg") | Some("jpg") => {
                    Some(convert_imagebytes_to_webpbytes(bytes).await)
                }
                Some("gz") => Some(unpack_targz(bytes.to_vec()).await),
                Some("zip") => todo!(),
                Some(&_) => todo!(),
                None => todo!(),
            },
        }
    }

    Ok((filename, filedata))
}
async fn convert_imagebytes_to_webpbytes(bytes: Bytes) -> Vec<u8> {
    let img2 = Reader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    img2.write_to(&mut Cursor::new(&mut bytes), ImageFormat::WebP)
        .unwrap();

    bytes
}

pub async fn unpack_targz(str: Vec<u8>) -> Vec<u8> {
    let ar = Archive::new(GzipDecoder::new(&str[..]));

    let id = generate_random_string();
    let target_folder = format!("uploads/{id}");

    create_dir(target_folder.to_owned()).await.unwrap();
    let mut entries = ar.entries().unwrap();
    while let Some(entry) = entries.next().await {
        let mut entry = entry.unwrap();
        entry.unpack_in(target_folder.to_owned()).await.unwrap();
    }

    let mut unconverted_entries = read_dir(target_folder).await.unwrap();

    let mut ar = Builder::new(Vec::new());
    while let Some(entry) = unconverted_entries.next_entry().await.unwrap() {
        let path = entry.path();
        let filename = entry.file_name();
        let mut file = File::open(path).await.unwrap();

        let mut vec: Vec<u8> = Vec::new();
        file.read_to_end(&mut vec).await.unwrap();

        let webp = convert_imagebytes_to_webpbytes(vec.into()).await;

        let mut new_filename = PathBuf::from(filename.clone());
        new_filename.set_extension("webp");

        let mut header = Header::new_gnu();
        header.set_path(new_filename).unwrap();
        header.set_size(webp.len().try_into().unwrap());
        header.set_cksum();

        ar.append(&header, &webp[..]).await.unwrap();
    }

    let e = ar.into_inner().await.unwrap();

    let mut writer = GzipEncoder::new(Vec::new());

    writer.write_all(&e).await.unwrap();
    writer.shutdown().await.unwrap();

    writer.into_inner()
}

pub async fn make_zipfile() -> impl IntoResponse {
    let filenamegen = format!("{}.zip", generate_random_string());

    let mut file = File::create(&filenamegen).await.unwrap();
    let mut writer = ZipFileWriter::with_tokio(&mut file);

    let data = tokio::fs::read("text.webp").await.unwrap();

    let builder = ZipEntryBuilder::new("meme.webp".into(), Compression::Deflate);
    writer.write_entry_whole(builder, &data).await.unwrap();
    writer.close().await.unwrap();

    let content_disposition = format!("attachment; filename = \"{filenamegen}\"");
    let mut header_map = HeaderMap::new();
    header_map.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );
    header_map.insert(
        header::CONTENT_DISPOSITION,
        content_disposition.parse().unwrap(),
    );

    let writer = tokio::fs::read(&filenamegen).await.unwrap();
    (header_map, writer)
}

pub enum ContentType {
    WebP,
    Gzip,
}

impl ContentType {
    fn get_content_type(&self) -> &'static str {
        match self {
            ContentType::WebP => "image/webp",
            ContentType::Gzip => "application/x-gzip",
        }
    }
}

fn create_header_map(filename: String, content: ContentType) -> HeaderMap {
    let content_type = content.get_content_type();

    let content_disposition = format!("attachment; filename = \"{filename}\"");
    let mut header_map = HeaderMap::new();
    header_map.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    header_map.insert(
        header::CONTENT_DISPOSITION,
        content_disposition.parse().unwrap(),
    );

    header_map
}

fn generate_random_string() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}
