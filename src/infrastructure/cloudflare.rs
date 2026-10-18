use s3::{Bucket, creds::Credentials, region::Region};

use crate::domain::R2;

#[allow(async_fn_in_trait)]
pub trait JsonFile {
    fn get_bucket(&self) -> Box<Bucket>;
    async fn upload(&self) -> bool;
    async fn delete(&self) -> bool;
    async fn download(&self) -> String;
}
impl JsonFile for R2 {
    async fn upload(&self) -> bool {
        !self
            .get_bucket()
            .put_object_with_content_type(self.path.clone(), &self.data, "application/gzip")
            .await
            .is_err()
        // .put_object_with_content_type(self.path.clone(), &self.data, "application/json")
        // .put_object_with_content_type(self.path.clone(), &self.data, "image/webp")
    }

    async fn delete(&self) -> bool {
        !self
            .get_bucket()
            .delete_object(self.path.clone())
            .await
            .is_err()
    }

    async fn download(&self) -> String {
        let res = self.get_bucket().get_object(self.path.clone()).await;

        match res {
            Ok(data) => String::from_utf8_lossy(data.as_slice()).to_string(),
            Err(_) => "".to_string(),
        }
    }

    fn get_bucket(&self) -> Box<Bucket> {
        let region = Region::Custom {
            region: "auto".into(),
            endpoint: self.url.clone(),
        };

        let creds = Credentials::new(
            Some(self.access.as_str()),
            Some(self.secret.as_str()),
            None,
            None,
            None,
        )
        .unwrap();

        Bucket::new(self.bucket.as_str(), region, creds)
            .unwrap()
            .with_path_style()
    }
}
