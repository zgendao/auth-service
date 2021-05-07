pub (crate) enum Internal {
    SetPassword,
    SetPermissions,
    SetInternalPermissions,
    GetPermissions,
    GetPermissionsOfEntity,
}

impl Internal {
    fn to_number(&self) -> u8 {
        match self {
            Internal::SetPassword => 1,
            Internal::SetPermissions => 2,
            Internal::SetInternalPermissions => 3,
            Internal::GetPermissions => 4,
            Internal::GetPermissionsOfEntity => 5,
        }
    }

    fn from_number(number: u8) -> Option<Internal> {
        match number {
            1 => Some(Internal::SetPassword),
            2 => Some(Internal::SetPermissions),
            3 => Some(Internal::SetInternalPermissions),
            4 => Some(Internal::GetPermissions),
            5 => Some(Internal::GetPermissionsOfEntity),
            _ => None,
        }
    }
}

pub (crate) struct InternalList {
    permissions: Vec<Internal>,
}