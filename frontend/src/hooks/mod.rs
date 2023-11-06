use crate::error::{FrontendError, FEResult};
use common::{from_str, StockMap, log_debug};
use gloo::net::http::Request;
use yew::{
    hook,
    suspense::{use_future, Suspension, SuspensionResult},
};

#[hook]
pub fn use_stock() -> SuspensionResult<FEResult<StockMap> > {
    let res: yew::suspense::UseFutureHandle<Result<StockMap, _>> = use_future(|| async {
        log_debug!("Requesting stock!");
        let req = Request::get("/api/stock/get").send().await?.text().await?;
        let stock: StockMap = from_str(&req)?;

        Ok::<StockMap, FrontendError>(stock)
    })?;

    match &(*res) {
        Ok(stock) => Ok(Ok(stock.clone())),
        Err(_) => Err(Suspension::new().0),
    }
}
