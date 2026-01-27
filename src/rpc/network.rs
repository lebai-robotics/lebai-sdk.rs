use super::Robot;
use cmod::Result;
use proto::lebai::network::*;

impl Robot {
    pub(crate) async fn http(&self, req: HttpRequest) -> Result<HttpResponse> {
        let resp = self.c.http(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
}
