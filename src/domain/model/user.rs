use crate::domain::model::line_user::LineUserProfile;
// use crate::domain::model::Id;
// pub struct User {
//     pub id: Id<User>,
//     pub auth_id: String as AuthUserId,
//     pub user_profile: UserProfile,
// }

// pub trait UserId {
//     fn value(&self) -> Id<&String>;
// }

pub trait User {
    fn user_id(&self) -> &String;
    fn user_profile(&self) -> UserProfile;
}

pub enum UserProfile {
    Line(LineUserProfile),
}
