pub struct Context;

#[cfg(test)]
mod test {
    use std::{sync::Arc, time::Duration};

    use serial_test::serial;
    use tokio::sync::OnceCell;

    use super::*;

    static CONTEXT: OnceCell<Arc<Context>> = OnceCell::const_new();

    async fn init_context() -> Arc<Context> {
        println!("creating shared context");
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("created shared context");
        Arc::new(Context)
    }

    #[tokio::test]
    #[serial]
    async fn test_one() {
        println!("running test_one");
        let _ctx = CONTEXT.get_or_init(init_context).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("completed test_one");
    }

    #[tokio::test]
    // #[serial]
    async fn test_two() {
        println!("running test_two");
        let _ctx = CONTEXT.get_or_init(init_context).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("completed test_two");
    }

    #[tokio::test]
    #[serial]
    async fn test_three() {
        println!("running test_three");
        let _ctx = CONTEXT.get_or_init(init_context).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("completed test_three");
    }

    #[tokio::test]
    #[serial]
    async fn test_four() {
        println!("running test_four");
        let _ctx = CONTEXT.get_or_init(init_context).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("completed test_four");
    }
}
