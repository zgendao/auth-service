pub const SET_PERMISSIONS: &str = "set_permissions";
pub const CREATE_PERMISSIONS: &str = "create_permissions";
pub const SET_INTERNAL_PERMISSIONS: &str = "set_internal_permissions";
pub const GET_USERS: &str = "get_users";
pub const GET_GROUPS: &str = "get_groups";
pub const CREATE_USER: &str = "create_user";
pub const GET_PERMISSIONS: &str = "get_permissions";

#[derive(Default)]
pub(crate) struct Permissions {
    pub(crate) set_permissions: bool,
    pub(crate) create_permissions: bool,
    pub(crate) set_internal_permissions: bool,
    pub(crate) get_users: bool,
    pub(crate) get_groups: bool,
    pub(crate) create_user: bool,
    pub(crate) get_permissions: bool,
}

impl Permissions {
    pub(crate) fn from(f: i64) -> Self {
        let mut p = Permissions::default();
        if f & (1 << 1) != 0 {
            p.set_permissions = true;
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
        if f & (1 << 6) != 0 {
            p.create_user = true;
        }
        if f & (1 << 7) != 0 {
            p.get_permissions = true;
        }
        p
    }

    pub(crate) fn from_vec(f: Vec<String>) -> Self {
        let mut p = Permissions::default();
        if f.contains(&SET_PERMISSIONS.to_string()) {
            p.set_permissions = true;
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
        if f.contains(&CREATE_USER.to_string()) {
            p.create_user = true;
        }
        if f.contains(&GET_PERMISSIONS.to_string()) {
            p.get_permissions = true;
        }
        p
    }

    pub(crate) fn to_number(&self) -> i64 {
        let mut n = 0;
        if self.set_permissions {
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
        if self.create_user {
            n = n + (1 << 6);
        }
        if self.get_permissions {
            n = n + (1 << 7);
        }
        n
    }

    pub(crate) fn to_vec(&self) -> Vec<String> {
        let mut n = Vec::<String>::new();
        if self.set_permissions {
            n.push(SET_PERMISSIONS.to_string())
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
        if self.create_user {
            n.push(CREATE_USER.to_string())
        }
        if self.get_permissions {
            n.push(GET_PERMISSIONS.to_string())
        }
        n
    }

    pub(crate) fn max() -> i64 {
        Permissions{
            set_permissions: true,
            create_permissions: true,
            set_internal_permissions: true,
            get_users: true,
            get_groups: true,
            create_user: true,
            get_permissions: true
        }.to_number()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::internal_permissions;

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
        assert_eq!(p.set_permissions, false);
        assert_eq!(p.create_permissions, true);
        assert_eq!(p.set_internal_permissions, false);
        assert_eq!(p.get_users, false);
        assert_eq!(p.get_groups, true);
        assert_eq!(p.create_user, false);
        assert_eq!(p.get_permissions, false);
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
        assert_eq!(p.set_permissions, false);
        assert_eq!(p.create_permissions, true);
        assert_eq!(p.set_internal_permissions, false);
        assert_eq!(p.get_users, false);
        assert_eq!(p.get_groups, true);
    }
}
