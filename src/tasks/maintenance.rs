use std::time::{Duration as StdDuration, SystemTime};
use tokio::fs::{read_dir, remove_dir};
use tokio::time::{sleep, Duration};

pub async fn delete_old_uploads() -> Result<(), String> {
    loop {
        let mut uploads_folder = read_dir("uploads").await.unwrap();
        while let Some(entry) = uploads_folder.next_entry().await.unwrap() {
            let metadata = entry.metadata().await.unwrap();
            let filepath = entry.path();
            let datetime_created = metadata.created().unwrap();
            if SystemTime::now().duration_since(datetime_created).unwrap()
                > StdDuration::from_secs(21600)
            {
                remove_dir(&filepath).await.unwrap();
            }
        }
        sleep(Duration::from_secs(600)).await;
    }

    eprintln!("Looks like something went wrong and the loop died :(");
    Ok(())
}
