use crate::error::FrontendError;
use common::{from_str, Stock};
use gloo::net::http::Request;
use yew::{
    hook,
    suspense::{use_future, Suspension, SuspensionResult},
};

#[hook]
pub fn use_stock() -> SuspensionResult<Result<Stock, FrontendError>> {
    let res = use_future(|| async {
        gloo::console::log!("Requesting stock!");
        let req = Request::get("/api/stock/get").send().await?.text().await?;
        let stock: Result<Stock, FrontendError> =
            from_str(&req).map_err(|e| FrontendError::DeserializationError(e.to_string()));
        if stock.is_err() {
            // T O D O :
            // Do exponential backoff or smth
            gloo::console::log!("Stock is error!!");
            return stock;
        }
        Ok(stock.unwrap())
    })?;

    match &(*res) {
        Ok(stock) => Ok(Ok(stock.clone())),
        Err(_) => Err(Suspension::new().0),
    }
}
