#[macro_export]
macro_rules! try_notice {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                notice::show(NoticeType::Error, &e);
                return;
            }
        }
    };
}
