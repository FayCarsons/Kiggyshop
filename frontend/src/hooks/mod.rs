use crate::error::{FEResult, FrontendError};
use common::{from_str, item::Item, log_debug, StockMap};
use gloo::net::http::Request;
use yew::{
    hook,
    suspense::{use_future, Suspension, SuspensionResult},
};

#[hook]
pub fn use_stock() -> SuspensionResult<FEResult<StockMap>> {
    let res: yew::suspense::UseFutureHandle<Result<StockMap, _>> = use_future(|| async {
        log_debug!("Requesting stock!");
        let req = Request::get("/api/stock/get").send().await?.text().await?;
        let stock = from_str::<StockMap>(&req)?;

        Ok::<StockMap, FrontendError>(stock)
    })?;

    match &(*res) {
        Ok(stock) => Ok(Ok(stock.clone())),
        Err(_) => Err(Suspension::new().0),
    }
}

#[hook]
pub fn use_item(item_id: &i32) -> SuspensionResult<FEResult<Item>> {
    let res = use_future(move || {
        let item_id = item_id.clone();
        async move {
            let url = format!("/api/stock/get_single/{item_id}");
            let req = Request::get(&url).send().await?.text().await?;
            let item = from_str::<Item>(&req)?;

            Ok::<Item, FrontendError>(item)
        }
    })?;

    match &(*res) {
        Ok(item) => Ok(Ok(item.clone())),
        Err(_) => Err(Suspension::new().0),
    }
}
