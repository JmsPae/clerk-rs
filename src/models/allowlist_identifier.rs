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
pub struct AllowlistIdentifier {
    /// String representing the object's type. Objects of the same type share the same value. 
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "invitation_id", skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    /// An email address or a phone number. 
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "identifier_type", skip_serializing_if = "Option::is_none")]
    pub identifier_type: Option<IdentifierType>,
    #[serde(rename = "instance_id", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Unix timestamp of creation 
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Unix timestamp of last update. 
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

impl AllowlistIdentifier {
    pub fn new() -> AllowlistIdentifier {
        AllowlistIdentifier {
            object: None,
            id: None,
            invitation_id: None,
            identifier: None,
            identifier_type: None,
            instance_id: None,
            created_at: None,
            updated_at: None,
        }
    }
}

/// String representing the object's type. Objects of the same type share the same value. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "allowlist_identifier")]
    AllowlistIdentifier,
}

impl Default for Object {
    fn default() -> Object {
        Self::AllowlistIdentifier
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentifierType {
    #[serde(rename = "email_address")]
    EmailAddress,
    #[serde(rename = "phone_number")]
    PhoneNumber,
    #[serde(rename = "web3_wallet")]
    Web3Wallet,
}

impl Default for IdentifierType {
    fn default() -> IdentifierType {
        Self::EmailAddress
    }
}

