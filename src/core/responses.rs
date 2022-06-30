pub enum Response {
    UnknownError,
    NoPermission,
    NotInThread,
    NoUsersInThread,
}

impl ToString for Response {
    fn to_string(&self) -> String {
        match &self {
            Response::UnknownError => String::from(
                "I'm sorry but it seems like there's something wrong :( I'll get this checked!",
            ),
            Response::NoPermission => String::from("Excuse me but you're not allowed to do that~"),
            Response::NotInThread => String::from("It seems like you're not in a thread, my dear~"),
            Response::NoUsersInThread => {
                String::from("Looks like there aren't any users in this thread~")
            }
        }
    }
}
