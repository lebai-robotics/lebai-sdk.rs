use super::Robot;
use cmod::Result;
use proto::lebai::storage::*;

impl Robot {
    pub(crate) async fn set_item(&self, key: String, value: String) -> Result<()> {
        let req = Item { key, value };
        let _ = self.c.set_item(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_item(&self, key: String) -> Result<Item> {
        let req = ItemIndex { key };
        let resp = self.c.get_item(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
    pub(crate) async fn get_items(&self, prefix: String) -> Result<Vec<Item>> {
        let req = GetItemsRequest { prefix };
        let resp = self.c.get_items(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.items)
    }
}
