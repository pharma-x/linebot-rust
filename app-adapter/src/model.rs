pub mod event;
pub mod line_user;
pub mod line_user_auth;
pub mod primary_user;
pub mod talk_room;

#[macro_export]
macro_rules! local_datetime {
    ($x:expr) => {
        chrono::Local
            .from_local_datetime(&NaiveDateTime::new($x.date(), $x.time()))
            .earliest()
            .ok_or(anyhow::anyhow!(
                "Cannot parse value {:?}, confirm your format.",
                $x
            ))?
    };
}
