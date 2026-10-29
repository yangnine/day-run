use s3::{Bucket, creds::Credentials, region::Region};

use crate::domain::R2;

#[allow(async_fn_in_trait)]
pub trait JsonFile {
    fn get_bucket(&self) -> Box<Bucket>;
    async fn upload(&self) -> bool;
    async fn delete(&self) -> bool;
    async fn download(&self) -> Option<Vec<u8>>;
    async fn list(&self) -> Vec<String>;
}
impl JsonFile for R2 {
    async fn upload(&self) -> bool {
        let bucket = self.get_bucket();
        // with_content_encoding 會驗證 header 值，所以回傳 Result，要先拆開再 execute
        let request = match bucket
            .put_object_builder(self.path.clone(), &self.data)
            .with_content_type("application/json")
            .with_content_encoding("gzip")
        {
            Ok(r) => r,
            Err(_) => return false,
        };
        request.execute().await.is_ok()
    }

    async fn delete(&self) -> bool {
        !self
            .get_bucket()
            .delete_object(self.path.clone())
            .await
            .is_err()
    }

    // 回傳原始 bytes：gzip 是 binary，轉 String 會壞掉
    async fn download(&self) -> Option<Vec<u8>> {
        let res = self.get_bucket().get_object(self.path.clone()).await.ok()?;
        Some(res.to_vec())
    }

    /// 列出 path (prefix) 底下所有 .gz 檔的 key
    async fn list(&self) -> Vec<String> {
        let pages = match self.get_bucket().list(self.path.clone(), None).await {
            Ok(p) => p,
            Err(_) => return vec![],
        };
        pages
            .into_iter()
            .flat_map(|page| page.contents)
            .map(|obj| obj.key)
            .filter(|key| key.ends_with(".gz"))
            .collect()
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
