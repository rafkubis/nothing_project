pub mod dummy_mqtt;
pub mod mqtt;
use crate::client;

pub trait MessageHandler<Msg: 'static> {
    fn handle_message(
        &mut self,
        msg: Msg,
        client: &(impl client::Client<Msg> + Send + Sync + 'static),
    ) -> impl std::future::Future<Output = ()> + Send + Sync;
}
