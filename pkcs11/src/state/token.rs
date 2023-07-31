type GroupId = Vec<u8>;
pub(crate) trait Token {}

#[derive(Default)]
pub(crate) struct MeesignToken {
    group_id: GroupId,
}

impl Token for MeesignToken {}
