use atspi::{
    connection::set_session_accessibility, events::object::ObjectEvents,
    proxy::accessible::AccessibleProxy, AccessibilityConnection, Event, GenericEvent,
};
use tokio_stream::StreamExt;
use zbus::proxy::CacheProperties;
use zvariant::ObjectPath;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_session_accessibility(true).await?;
    let conn = AccessibilityConnection::new().await?;

    conn.register_event::<ObjectEvents>().await?;

    let events = conn.event_stream();
    tokio::pin!(events);

    while let Some(Ok(ref ev)) = events.next().await {
        if let Event::Object(ObjectEvents::ChildrenChanged(ref event)) = ev {
            // sender() and path() are provided by the GenericEvent trait
            let sender = event.sender();
            let path: ObjectPath = event.path();

            let accesible_proxy = AccessibleProxy::builder(conn.connection())
                .path(path)?
                .destination(sender)?
                .cache_properties(CacheProperties::No)
                .build()
                .await?;

            let name = accesible_proxy.name().await?;
            println!("Name of the object that sent a children-changed event: {name}");
            let children = accesible_proxy.get_children().await?;
            for child in children {
                let child_name = child.name;
                println!("Child name: {child_name}");
            }
        }
    }

    Ok(())
}
