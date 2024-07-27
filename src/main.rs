use std::future::Future;

trait DesugaredAsyncTrait {
    fn foo(&self) -> impl Future<Output = usize> + Send;
    fn bar(&self) -> impl Future<Output = usize> + Send;
}

struct Foo;

impl DesugaredAsyncTrait for Foo {
    fn foo(&self) -> impl Future<Output = usize> + Send {
        async { 1 }
    }

    // 
    async fn bar(&self) -> usize {
        1
    }
}

fn main() {
    let fut = Foo.bar();
    fn _assert_send<T: Send>(_: T) {}
    _assert_send(fut);
}