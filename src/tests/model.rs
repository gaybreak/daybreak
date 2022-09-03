use time::OffsetDateTime;

use crate::model::{
    permission::{self, Permissions},
    Id,
};

#[test]
fn id_timestamp() {
    assert_eq!(
        Id(258_568_289_746_288_641).timestamp().unwrap(),
        OffsetDateTime::from_unix_timestamp(1_481_717_884).unwrap()
    );
}

#[test]
fn permissions_pretty_string() {
    assert_eq!(
        permission::to_pretty_string(Permissions::CreateInstantInvite | Permissions::KickMembers),
        "- Create Invite\n- Kick Members"
    );
}
