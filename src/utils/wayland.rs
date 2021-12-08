use swayipc::reply;

pub fn get_window_class(evt: Option<Result<reply::Event, swayipc::Error>>) -> String {
    match evt {
        Some(Ok(reply::Event::Window(w))) => {
            // app_id => native wayland
            // xwayland => window_properties.class
            match (w.container.app_id, w.container.window_properties) {
                (Some(id), _) => id,
                (_, Some(props)) => match props.class {
                    Some(class) => class,
                    _ => panic!("Cannot get window id"),
                },
                (_, _) => panic!("Cannot get window id"),
            }
        }
        _ => panic!("Cannot get window id"),
    }
}
