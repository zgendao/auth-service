pub const SET_PASSWORDS: &str = "set_passwords";
pub const CREATE_PERMISSIONS: &str = "create_permissions";
pub const SET_INTERNAL_PERMISSIONS: &str = "set_internal_permissions";
pub const GET_USERS: &str = "get_users";
pub const GET_GROUPS: &str = "get_groups";

#[derive(Default)]
pub(crate) struct Permissions {
    pub(crate) set_passwords: bool,
    pub(crate) create_permissions: bool,
    pub(crate) set_internal_permissions: bool,
    pub(crate) get_users: bool,
    pub(crate) get_groups: bool,
}

impl Permissions {
    pub(crate) fn from(f: u64) -> Self {
        let mut p = Permissions::default();
        if f & (1 << 1) != 0 {
            p.set_passwords = true;
        }
        if f & (1 << 2) != 0 {
            p.create_permissions = true;
        }
        if f & (1 << 3) != 0 {
            p.set_internal_permissions = true;
        }
        if f & (1 << 4) != 0 {
            p.get_users = true;
        }
        if f & (1 << 5) != 0 {
            p.get_groups = true;
        }
        p
    }

    pub(crate) fn from_vec(f: Vec<String>) -> Self {
        let mut p = Permissions::default();
        if f.contains(&SET_PASSWORDS.to_string()) {
            p.set_passwords = true;
        }
        if f.contains(&CREATE_PERMISSIONS.to_string()) {
            p.create_permissions = true;
        }
        if f.contains(&SET_INTERNAL_PERMISSIONS.to_string()) {
            p.set_internal_permissions = true;
        }
        if f.contains(&GET_USERS.to_string()) {
            p.get_users = true;
        }
        if f.contains(&GET_GROUPS.to_string()) {
            p.get_groups = true;
        }
        p
    }

    pub(crate) fn to_number(self) -> u64 {
        let mut n = 0;
        if self.set_passwords {
            n = n + (1 << 1);
        }
        if self.create_permissions {
            n = n + (1 << 2);
        }
        if self.set_internal_permissions {
            n = n + (1 << 3);
        }
        if self.get_users {
            n = n + (1 << 4);
        }
        if self.get_groups {
            n = n + (1 << 5);
        }
        n
    }

    pub(crate) fn to_vec(self) -> Vec<String> {
        let mut n = Vec::<String>::new();
        if self.set_passwords {
            n.push(SET_PASSWORDS.to_string())
        }
        if self.create_permissions {
            n.push(CREATE_PERMISSIONS.to_string())
        }
        if self.set_internal_permissions {
            n.push(SET_INTERNAL_PERMISSIONS.to_string())
        }
        if self.get_users {
            n.push(GET_USERS.to_string())
        }
        if self.get_groups {
            n.push(GET_GROUPS.to_string())
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use crate::core::internal_permissions;
    use crate::core::internal_permissions::CREATE_PERMISSIONS;

    #[test]
    fn test_permissions_to_number() {
        let mut p = internal_permissions::Permissions::default();
        p.create_permissions = true;
        p.get_groups = true;
        assert_eq!(p.to_number(), 36);
    }

    #[test]
    fn test_permissions_from() {
        let p = internal_permissions::Permissions::from(36);
        assert_eq!(p.set_passwords, false);
        assert_eq!(p.create_permissions, true);
        assert_eq!(p.set_internal_permissions, false);
        assert_eq!(p.get_users, false);
        assert_eq!(p.get_groups, true);
    }

    #[test]
    fn test_permissions_vec() {
        let mut p = internal_permissions::Permissions::default();
        p.create_permissions = true;
        p.get_groups = true;

        let v_p = p.to_vec();
        let mut v = Vec::<String>::new();
        v.push(internal_permissions::CREATE_PERMISSIONS.parse().unwrap());
        v.push(internal_permissions::GET_GROUPS.parse().unwrap());
        assert_eq!(v, v_p);

        let p = internal_permissions::Permissions::from_vec(v_p);
        assert_eq!(p.set_passwords, false);
        assert_eq!(p.create_permissions, true);
        assert_eq!(p.set_internal_permissions, false);
        assert_eq!(p.get_users, false);
        assert_eq!(p.get_groups, true);
    }
}
