use std::future::Future;

macro_rules! compose_middleware_inner {
  ( $route:ident, $first:ident, $second:ident, $($tail:ident), +) => {
    $first(|| async {
        compose_middleware_inner!($route, $second, $($tail),+)
    }).await
  };
  ( $route: ident, $first:ident, $second:ident ) => {
    $first(|| async move { $second($route).await }).await
  };
}

/// Replacing the macro with its expanded form still results in huge compile times.
/// The macro will generate a function that looks like this:
/// 
/// pub async fn my_middleware<N, Fut>(route: N)
/// where
///     N: FnOnce() -> Fut,
///     Fut: Future<Output = ()>,
/// {
///     log(|| async { log(|| async { log(|| async move { log(route).await }).await }).await }).await
/// }
macro_rules! compose_middleware {
    ( $name:ident, $($tail:ident), +) => {
        pub async fn $name<N, Fut>(route: N)
        where
            N: FnOnce() -> Fut,
            Fut: Future<Output = ()>,
        {
            compose_middleware_inner!(route, $($tail),+)
        }
    }
}

async fn log<N, Fut>(next: N)
where
    N: FnOnce() -> Fut,
    Fut: Future<Output = ()>,
{
    println!("log start");
    next().await;
    println!("log end");
}

compose_middleware!(
    my_middleware,
    log,
    log,
    log,
    log,
    log,
    log,
    log,
    log,
    log,
    log,
    log,
    log,
    log,
    log,
    log
);
