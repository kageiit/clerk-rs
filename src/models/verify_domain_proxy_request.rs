/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyDomainProxyRequest {
	/// The ID of the domain that will be updated.
	#[serde(rename = "domain_id", skip_serializing_if = "Option::is_none")]
	pub domain_id: Option<String>,
	/// The full URL of the proxy which will forward requests to the Clerk Frontend API for this domain. e.g. https://example.com/__clerk
	#[serde(rename = "proxy_url", skip_serializing_if = "Option::is_none")]
	pub proxy_url: Option<String>,
}

impl VerifyDomainProxyRequest {
	pub fn new() -> VerifyDomainProxyRequest {
		VerifyDomainProxyRequest {
			domain_id: None,
			proxy_url: None,
		}
	}
}