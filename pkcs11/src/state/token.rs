use crate::communicator::GroupId;

pub(crate) trait Token {}

#[derive(Default)]
pub(crate) struct MeesignToken {
    group_id: GroupId,
}

impl Token for MeesignToken {}

impl MeesignToken {
    pub(crate) fn new(group_id: GroupId) -> Self {
        Self { group_id }
    }
}
