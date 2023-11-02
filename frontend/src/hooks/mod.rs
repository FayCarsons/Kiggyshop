use crate::error::FrontendError;
use common::{from_str, item::Item, StockMap};
use gloo::net::http::Request;
use yew::{
    hook,
    suspense::{use_future, Suspension, SuspensionResult},
};

#[hook]
pub fn use_stock() -> SuspensionResult<Result<StockMap, FrontendError>> {
    let res: yew::suspense::UseFutureHandle<Result<StockMap, _>> = use_future(|| async {
        gloo::console::log!("Requesting stock!");
        let req = Request::get("/api/stock/get").send().await?.text().await?;
        let stock: Result<StockMap, FrontendError> =
            from_str(&req).map_err(|e| FrontendError::DeserializationError(e.to_string()));
        if stock.is_err() {
            // !! T O D O !!
            // Do exponential backoff or smth
            gloo::console::log!("Stock is error!!");
        }
        Ok::<StockMap, FrontendError>(stock.unwrap())
    })?;

    match &(*res) {
        Ok(stock) => Ok(Ok(stock.clone())),
        Err(_) => Err(Suspension::new().0),
    }
}
