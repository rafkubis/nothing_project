pub mod dummy_mqtt;
pub mod mqtt;

pub trait MessageHandler<Msg: 'static> {
    fn handle_message(&mut self, msg: Msg) -> impl std::future::Future<Output = ()> + Send + Sync;
}
