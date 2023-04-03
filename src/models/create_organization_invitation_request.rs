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
pub struct CreateOrganizationInvitationRequest {
    /// The email address of the new member that is going to be invited to the organization
    #[serde(rename = "email_address")]
    pub email_address: String,
    /// The ID of the user that invites the new member to the organization. Must be an administrator in the organization.
    #[serde(rename = "inviter_user_id")]
    pub inviter_user_id: String,
    /// The role of the new member in the organization
    #[serde(rename = "role")]
    pub role: Role,
    /// Metadata saved on the organization invitation, read-only from the Frontend API and fully accessible (read/write) from the Backend API.
    #[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
    pub public_metadata: Option<serde_json::Value>,
    /// Optional URL that the invitee will be redirected to once they accept the invitation by clicking the join link in the invitation email.
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

impl CreateOrganizationInvitationRequest {
    pub fn new(email_address: String, inviter_user_id: String, role: Role) -> CreateOrganizationInvitationRequest {
        CreateOrganizationInvitationRequest {
            email_address,
            inviter_user_id,
            role,
            public_metadata: None,
            redirect_url: None,
        }
    }
}

/// The role of the new member in the organization
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

