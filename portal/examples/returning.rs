use portal::{filters::BoxedFilter, Filter, Rejection, Reply};

// Option 1: BoxedFilter
// Note that this may be useful for shortening compile times when you are composing many filters.
// Boxing the filters will use dynamic dispatch and speed up compilation while
// making it slightly slower at runtime.
pub fn assets_filter() -> BoxedFilter<(impl Reply,)> {
    portal::path("assets").and(portal::fs::dir("./assets")).boxed()
}

// Option 2: impl Filter + Clone
pub fn index_filter() -> impl Filter<Extract = (&'static str,), Error = Rejection> + Clone {
    portal::path::end().map(|| "Index page")
}

#[tokio::main]
async fn main() {
    let routes = index_filter().or(assets_filter());
    portal::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
