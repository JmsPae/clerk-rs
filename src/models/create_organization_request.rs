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
pub struct CreateOrganizationRequest {
    /// The name of the new organization
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the User who will become the administrator for the new organization
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Metadata saved on the organization, accessible only from the Backend API
    #[serde(rename = "private_metadata", skip_serializing_if = "Option::is_none")]
    pub private_metadata: Option<serde_json::Value>,
    /// Metadata saved on the organization, read-only from the Frontend API and fully accessible (read/write) from the Backend API
    #[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
    pub public_metadata: Option<serde_json::Value>,
    /// A slug for the new organization. Can contain only lowercase alphanumeric characters and the dash \"-\". Must be unique for the instance.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// The maximum number of memberships allowed for this organization
    #[serde(rename = "max_allowed_memberships", skip_serializing_if = "Option::is_none")]
    pub max_allowed_memberships: Option<i32>,
}

impl CreateOrganizationRequest {
    pub fn new(name: String, created_by: String) -> CreateOrganizationRequest {
        CreateOrganizationRequest {
            name,
            created_by,
            private_metadata: None,
            public_metadata: None,
            slug: None,
            max_allowed_memberships: None,
        }
    }
}


