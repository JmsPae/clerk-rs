/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateOrganizationMembershipRequest {
    /// The ID of the user that will be added as a member in the organization. The user needs to exist in the same instance as the organization and must not be a member of the given organization already.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// The role that the new member will have in the organization.
    #[serde(rename = "role")]
    pub role: Role,
}

impl CreateOrganizationMembershipRequest {
    pub fn new(user_id: String, role: Role) -> CreateOrganizationMembershipRequest {
        CreateOrganizationMembershipRequest {
            user_id,
            role,
        }
    }
}

/// The role that the new member will have in the organization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "basic_member")]
    BasicMember,
}

impl Default for Role {
    fn default() -> Role {
        Self::Admin
    }
}

