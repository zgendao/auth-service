pub const MANAGE_PERMISSIONS: &str = "manage_permissions"; // << 1
pub const MANAGE_USERS: &str = "manage_users"; // << 2
pub const MANAGE_GROUPS: &str = "manage_groups"; // << 3
pub const MANAGE_TOKENS: &str = "manage_tokens"; // << 4
pub const SET_INTERNAL_PERMISSIONS: &str = "set_internal_permissions"; // << 5
pub const GET_USERS: &str = "get_users"; // << 6
pub const GET_GROUPS: &str = "get_groups"; // << 7
pub const GET_PERMISSIONS: &str = "get_permissions"; // << 8
pub const MANAGE_LONG_TOKEN: &str = "manage_long_token"; // << 9

#[derive(Default)]
pub struct Permissions {
    pub manage_permissions: bool,
    pub manage_users: bool,
    pub manage_groups: bool,
    pub manage_tokens: bool,
    pub set_internal_permissions: bool,
    pub get_users: bool,
    pub get_groups: bool,
    pub get_permissions: bool,
    pub manage_long_token: bool,
}

impl Permissions {
    pub fn from(f: i64) -> Self {
        let mut p = Permissions::default();
        if f & (1 << 1) != 0 {
            p.manage_permissions = true;
        }
        if f & (1 << 2) != 0 {
            p.manage_users = true;
        }
        if f & (1 << 3) != 0 {
            p.manage_groups = true;
        }
        if f & (1 << 4) != 0 {
            p.manage_tokens = true;
        }
        if f & (1 << 5) != 0 {
            p.set_internal_permissions = true;
        }
        if f & (1 << 6) != 0 {
            p.get_users = true;
        }
        if f & (1 << 7) != 0 {
            p.get_groups = true;
        }
        if f & (1 << 8) != 0 {
            p.get_permissions = true;
        }
        if f & (1 << 9) != 0 {
            p.manage_long_token = true;
        }
        p
    }

    pub fn from_vec(f: Vec<String>) -> Self {
        let mut p = Permissions::default();
        if f.iter().any(|item| item == MANAGE_PERMISSIONS) {
            p.manage_permissions = true;
        }
        if f.iter().any(|item| item == MANAGE_USERS) {
            p.manage_users = true;
        }
        if f.iter().any(|item| item == MANAGE_GROUPS) {
            p.manage_groups = true;
        }
        if f.iter().any(|item| item == MANAGE_TOKENS) {
            p.manage_tokens = true;
        }
        if f.iter().any(|item| item == SET_INTERNAL_PERMISSIONS) {
            p.set_internal_permissions = true;
        }
        if f.iter().any(|item| item == GET_USERS) {
            p.get_users = true;
        }
        if f.iter().any(|item| item == GET_GROUPS) {
            p.get_groups = true;
        }
        if f.iter().any(|item| item == GET_PERMISSIONS) {
            p.get_permissions = true;
        }
        if f.iter().any(|item| item == MANAGE_LONG_TOKEN) {
            p.manage_long_token = true;
        }
        p
    }

    pub fn to_number(&self) -> i64 {
        let mut n = 0;
        if self.manage_permissions {
            n += 1 << 1;
        }
        if self.manage_users {
            n += 1 << 2;
        }
        if self.manage_groups {
            n += 1 << 3;
        }
        if self.manage_tokens {
            n += 1 << 4;
        }
        if self.set_internal_permissions {
            n += 1 << 5;
        }
        if self.get_users {
            n += 1 << 6;
        }
        if self.get_groups {
            n += 1 << 7;
        }
        if self.get_permissions {
            n += 1 << 8;
        }
        if self.manage_long_token {
            n += 1 << 9;
        }
        n
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut n = Vec::<String>::new();
        if self.manage_permissions {
            n.push(MANAGE_PERMISSIONS.to_string())
        }
        if self.manage_users {
            n.push(MANAGE_USERS.to_string())
        }
        if self.manage_groups {
            n.push(MANAGE_GROUPS.to_string())
        }
        if self.manage_tokens {
            n.push(MANAGE_TOKENS.to_string())
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
        if self.get_permissions {
            n.push(GET_PERMISSIONS.to_string())
        }
        if self.manage_long_token {
            n.push(MANAGE_LONG_TOKEN.to_string())
        }
        n
    }

    pub fn max() -> i64 {
        Permissions {
            manage_permissions: true,
            manage_users: true,
            manage_groups: true,
            manage_tokens: true,
            set_internal_permissions: true,
            get_users: true,
            get_groups: true,
            get_permissions: true,
            manage_long_token: true,
        }
        .to_number()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::internal_permissions;

    #[test]
    fn test_permissions_to_number() {
        let mut p = internal_permissions::Permissions::default();
        p.manage_permissions = true;
        p.get_groups = true;
        assert_eq!(p.to_number(), 130);
    }

    #[test]
    fn test_permissions_from() {
        let p = internal_permissions::Permissions::from(130);
        assert_eq!(p.manage_permissions, true);
        assert_eq!(p.manage_users, false);
        assert_eq!(p.manage_groups, false);
        assert_eq!(p.manage_tokens, false);
        assert_eq!(p.set_internal_permissions, false);
        assert_eq!(p.get_users, false);
        assert_eq!(p.get_groups, true);
        assert_eq!(p.get_permissions, false);
        assert_eq!(p.manage_long_token, false);
    }

    #[test]
    fn test_permissions_vec() {
        let mut p = internal_permissions::Permissions::default();
        p.manage_permissions = true;
        p.get_groups = true;

        let v_p = p.to_vec();
        let mut v = Vec::<String>::new();
        v.push(internal_permissions::MANAGE_PERMISSIONS.parse().unwrap());
        v.push(internal_permissions::GET_GROUPS.parse().unwrap());
        assert_eq!(v, v_p);

        let p = internal_permissions::Permissions::from_vec(v_p);
        assert_eq!(p.manage_permissions, true);
        assert_eq!(p.set_internal_permissions, false);
        assert_eq!(p.get_users, false);
        assert_eq!(p.get_groups, true);
    }
}
