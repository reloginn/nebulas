pub trait Sleep {
    type T: Send + Sync + 'static;

    fn sleep(duration: Self::T) -> impl std::future::Future<Output = ()> + Send;
}
