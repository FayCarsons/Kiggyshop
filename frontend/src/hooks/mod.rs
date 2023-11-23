use crate::error::{FEResult, FrontendError};
use common::{
    from_str,
    item::{FrontEndItem, Item},
    log_debug, StockMap,
};
use gloo::{net::http::Request, console::log};
use yew::{
    hook,
    suspense::{use_future, Suspension, SuspensionResult},
};
use std::rc::Rc;

#[hook]
pub fn use_stock() -> SuspensionResult<FEResult<Rc<StockMap>>> {
    let res: yew::suspense::UseFutureHandle<Result<StockMap, _>> = use_future(|| async {
        log_debug!("Requesting stock!");
        let req = Request::get("/api/stock/get").send().await?.text().await?;
        let stock = from_str::<Vec<Item>>(&req)?;
        let map = stock
            .iter()
            .map(|item| (item.id as u32, FrontEndItem::from(item)))
            .collect::<StockMap>();

        Ok::<StockMap, FrontendError>(map)
    })?;

    match &(*res) {
        Ok(stock) => {
            let mut stock = stock.clone();
            stock.shrink_to_fit();
            let stock = Rc::new(stock);
            Ok(Ok(stock))
        },
        Err(_) => {
            log!("use_stock went to messy match arm, creating new suspension ??");
            Err(Suspension::new().0)
        },
    }
}
