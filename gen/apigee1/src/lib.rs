// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Apigee* crate version *2.0.8+20210319*, where *20210319* is the exact revision of the *apigee:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v2.0.8*.
//! 
//! Everything else about the *Apigee* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/apigee-api-management/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/apigee1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Apigee) ... 
//! 
//! * hybrid
//!  * [*issuers list*](api::HybridIssuerListCall)
//! * organizations
//!  * [*analytics datastores create*](api::OrganizationAnalyticDatastoreCreateCall), [*analytics datastores delete*](api::OrganizationAnalyticDatastoreDeleteCall), [*analytics datastores get*](api::OrganizationAnalyticDatastoreGetCall), [*analytics datastores list*](api::OrganizationAnalyticDatastoreListCall), [*analytics datastores test*](api::OrganizationAnalyticDatastoreTestCall), [*analytics datastores update*](api::OrganizationAnalyticDatastoreUpdateCall), [*apiproducts attributes*](api::OrganizationApiproductAttributeCall), [*apiproducts attributes delete*](api::OrganizationApiproductAttributeDeleteCall), [*apiproducts attributes get*](api::OrganizationApiproductAttributeGetCall), [*apiproducts attributes list*](api::OrganizationApiproductAttributeListCall), [*apiproducts attributes update api product attribute*](api::OrganizationApiproductAttributeUpdateApiProductAttributeCall), [*apiproducts create*](api::OrganizationApiproductCreateCall), [*apiproducts delete*](api::OrganizationApiproductDeleteCall), [*apiproducts get*](api::OrganizationApiproductGetCall), [*apiproducts list*](api::OrganizationApiproductListCall), [*apiproducts update*](api::OrganizationApiproductUpdateCall), [*apis create*](api::OrganizationApiCreateCall), [*apis delete*](api::OrganizationApiDeleteCall), [*apis deployments list*](api::OrganizationApiDeploymentListCall), [*apis get*](api::OrganizationApiGetCall), [*apis keyvaluemaps create*](api::OrganizationApiKeyvaluemapCreateCall), [*apis keyvaluemaps delete*](api::OrganizationApiKeyvaluemapDeleteCall), [*apis list*](api::OrganizationApiListCall), [*apis revisions delete*](api::OrganizationApiRevisionDeleteCall), [*apis revisions deployments list*](api::OrganizationApiRevisionDeploymentListCall), [*apis revisions get*](api::OrganizationApiRevisionGetCall), [*apis revisions update api proxy revision*](api::OrganizationApiRevisionUpdateApiProxyRevisionCall), [*apps get*](api::OrganizationAppGetCall), [*apps list*](api::OrganizationAppListCall), [*create*](api::OrganizationCreateCall), [*datacollectors create*](api::OrganizationDatacollectorCreateCall), [*datacollectors delete*](api::OrganizationDatacollectorDeleteCall), [*datacollectors get*](api::OrganizationDatacollectorGetCall), [*datacollectors list*](api::OrganizationDatacollectorListCall), [*datacollectors patch*](api::OrganizationDatacollectorPatchCall), [*delete*](api::OrganizationDeleteCall), [*deployments list*](api::OrganizationDeploymentListCall), [*developers apps attributes*](api::OrganizationDeveloperAppAttributeCall), [*developers apps attributes delete*](api::OrganizationDeveloperAppAttributeDeleteCall), [*developers apps attributes get*](api::OrganizationDeveloperAppAttributeGetCall), [*developers apps attributes list*](api::OrganizationDeveloperAppAttributeListCall), [*developers apps attributes update developer app attribute*](api::OrganizationDeveloperAppAttributeUpdateDeveloperAppAttributeCall), [*developers apps create*](api::OrganizationDeveloperAppCreateCall), [*developers apps delete*](api::OrganizationDeveloperAppDeleteCall), [*developers apps generate key pair or update developer app status*](api::OrganizationDeveloperAppGenerateKeyPairOrUpdateDeveloperAppStatuCall), [*developers apps get*](api::OrganizationDeveloperAppGetCall), [*developers apps keys apiproducts delete*](api::OrganizationDeveloperAppKeyApiproductDeleteCall), [*developers apps keys apiproducts update developer app key api product*](api::OrganizationDeveloperAppKeyApiproductUpdateDeveloperAppKeyApiProductCall), [*developers apps keys create*](api::OrganizationDeveloperAppKeyCreateCall), [*developers apps keys create create*](api::OrganizationDeveloperAppKeyCreateCreateCall), [*developers apps keys delete*](api::OrganizationDeveloperAppKeyDeleteCall), [*developers apps keys get*](api::OrganizationDeveloperAppKeyGetCall), [*developers apps keys replace developer app key*](api::OrganizationDeveloperAppKeyReplaceDeveloperAppKeyCall), [*developers apps keys update developer app key*](api::OrganizationDeveloperAppKeyUpdateDeveloperAppKeyCall), [*developers apps list*](api::OrganizationDeveloperAppListCall), [*developers apps update*](api::OrganizationDeveloperAppUpdateCall), [*developers attributes*](api::OrganizationDeveloperAttributeCall), [*developers attributes delete*](api::OrganizationDeveloperAttributeDeleteCall), [*developers attributes get*](api::OrganizationDeveloperAttributeGetCall), [*developers attributes list*](api::OrganizationDeveloperAttributeListCall), [*developers attributes update developer attribute*](api::OrganizationDeveloperAttributeUpdateDeveloperAttributeCall), [*developers create*](api::OrganizationDeveloperCreateCall), [*developers delete*](api::OrganizationDeveloperDeleteCall), [*developers get*](api::OrganizationDeveloperGetCall), [*developers list*](api::OrganizationDeveloperListCall), [*developers set developer status*](api::OrganizationDeveloperSetDeveloperStatuCall), [*developers update*](api::OrganizationDeveloperUpdateCall), [*envgroups attachments create*](api::OrganizationEnvgroupAttachmentCreateCall), [*envgroups attachments delete*](api::OrganizationEnvgroupAttachmentDeleteCall), [*envgroups attachments get*](api::OrganizationEnvgroupAttachmentGetCall), [*envgroups attachments list*](api::OrganizationEnvgroupAttachmentListCall), [*envgroups create*](api::OrganizationEnvgroupCreateCall), [*envgroups delete*](api::OrganizationEnvgroupDeleteCall), [*envgroups get*](api::OrganizationEnvgroupGetCall), [*envgroups list*](api::OrganizationEnvgroupListCall), [*envgroups patch*](api::OrganizationEnvgroupPatchCall), [*environments analytics admin get schemav2*](api::OrganizationEnvironmentAnalyticAdminGetSchemav2Call), [*environments analytics exports create*](api::OrganizationEnvironmentAnalyticExportCreateCall), [*environments analytics exports get*](api::OrganizationEnvironmentAnalyticExportGetCall), [*environments analytics exports list*](api::OrganizationEnvironmentAnalyticExportListCall), [*environments apis deployments list*](api::OrganizationEnvironmentApiDeploymentListCall), [*environments apis revisions debugsessions create*](api::OrganizationEnvironmentApiRevisionDebugsessionCreateCall), [*environments apis revisions debugsessions data get*](api::OrganizationEnvironmentApiRevisionDebugsessionDataGetCall), [*environments apis revisions debugsessions delete data*](api::OrganizationEnvironmentApiRevisionDebugsessionDeleteDataCall), [*environments apis revisions debugsessions get*](api::OrganizationEnvironmentApiRevisionDebugsessionGetCall), [*environments apis revisions debugsessions list*](api::OrganizationEnvironmentApiRevisionDebugsessionListCall), [*environments apis revisions deploy*](api::OrganizationEnvironmentApiRevisionDeployCall), [*environments apis revisions deployments generate deploy change report*](api::OrganizationEnvironmentApiRevisionDeploymentGenerateDeployChangeReportCall), [*environments apis revisions deployments generate undeploy change report*](api::OrganizationEnvironmentApiRevisionDeploymentGenerateUndeployChangeReportCall), [*environments apis revisions get deployments*](api::OrganizationEnvironmentApiRevisionGetDeploymentCall), [*environments apis revisions undeploy*](api::OrganizationEnvironmentApiRevisionUndeployCall), [*environments caches delete*](api::OrganizationEnvironmentCacheDeleteCall), [*environments create*](api::OrganizationEnvironmentCreateCall), [*environments delete*](api::OrganizationEnvironmentDeleteCall), [*environments deployments list*](api::OrganizationEnvironmentDeploymentListCall), [*environments flowhooks attach shared flow to flow hook*](api::OrganizationEnvironmentFlowhookAttachSharedFlowToFlowHookCall), [*environments flowhooks detach shared flow from flow hook*](api::OrganizationEnvironmentFlowhookDetachSharedFlowFromFlowHookCall), [*environments flowhooks get*](api::OrganizationEnvironmentFlowhookGetCall), [*environments get*](api::OrganizationEnvironmentGetCall), [*environments get debugmask*](api::OrganizationEnvironmentGetDebugmaskCall), [*environments get deployed config*](api::OrganizationEnvironmentGetDeployedConfigCall), [*environments get iam policy*](api::OrganizationEnvironmentGetIamPolicyCall), [*environments get trace config*](api::OrganizationEnvironmentGetTraceConfigCall), [*environments keystores aliases create*](api::OrganizationEnvironmentKeystoreAliaseCreateCall), [*environments keystores aliases csr*](api::OrganizationEnvironmentKeystoreAliaseCsrCall), [*environments keystores aliases delete*](api::OrganizationEnvironmentKeystoreAliaseDeleteCall), [*environments keystores aliases get*](api::OrganizationEnvironmentKeystoreAliaseGetCall), [*environments keystores aliases get certificate*](api::OrganizationEnvironmentKeystoreAliaseGetCertificateCall), [*environments keystores aliases update*](api::OrganizationEnvironmentKeystoreAliaseUpdateCall), [*environments keystores create*](api::OrganizationEnvironmentKeystoreCreateCall), [*environments keystores delete*](api::OrganizationEnvironmentKeystoreDeleteCall), [*environments keystores get*](api::OrganizationEnvironmentKeystoreGetCall), [*environments keyvaluemaps create*](api::OrganizationEnvironmentKeyvaluemapCreateCall), [*environments keyvaluemaps delete*](api::OrganizationEnvironmentKeyvaluemapDeleteCall), [*environments optimized stats get*](api::OrganizationEnvironmentOptimizedStatGetCall), [*environments queries create*](api::OrganizationEnvironmentQueryCreateCall), [*environments queries get*](api::OrganizationEnvironmentQueryGetCall), [*environments queries get result*](api::OrganizationEnvironmentQueryGetResultCall), [*environments queries list*](api::OrganizationEnvironmentQueryListCall), [*environments references create*](api::OrganizationEnvironmentReferenceCreateCall), [*environments references delete*](api::OrganizationEnvironmentReferenceDeleteCall), [*environments references get*](api::OrganizationEnvironmentReferenceGetCall), [*environments references update*](api::OrganizationEnvironmentReferenceUpdateCall), [*environments resourcefiles create*](api::OrganizationEnvironmentResourcefileCreateCall), [*environments resourcefiles delete*](api::OrganizationEnvironmentResourcefileDeleteCall), [*environments resourcefiles get*](api::OrganizationEnvironmentResourcefileGetCall), [*environments resourcefiles list*](api::OrganizationEnvironmentResourcefileListCall), [*environments resourcefiles list environment resources*](api::OrganizationEnvironmentResourcefileListEnvironmentResourceCall), [*environments resourcefiles update*](api::OrganizationEnvironmentResourcefileUpdateCall), [*environments set iam policy*](api::OrganizationEnvironmentSetIamPolicyCall), [*environments sharedflows deployments list*](api::OrganizationEnvironmentSharedflowDeploymentListCall), [*environments sharedflows revisions deploy*](api::OrganizationEnvironmentSharedflowRevisionDeployCall), [*environments sharedflows revisions get deployments*](api::OrganizationEnvironmentSharedflowRevisionGetDeploymentCall), [*environments sharedflows revisions undeploy*](api::OrganizationEnvironmentSharedflowRevisionUndeployCall), [*environments stats get*](api::OrganizationEnvironmentStatGetCall), [*environments subscribe*](api::OrganizationEnvironmentSubscribeCall), [*environments targetservers create*](api::OrganizationEnvironmentTargetserverCreateCall), [*environments targetservers delete*](api::OrganizationEnvironmentTargetserverDeleteCall), [*environments targetservers get*](api::OrganizationEnvironmentTargetserverGetCall), [*environments targetservers update*](api::OrganizationEnvironmentTargetserverUpdateCall), [*environments test iam permissions*](api::OrganizationEnvironmentTestIamPermissionCall), [*environments trace config overrides create*](api::OrganizationEnvironmentTraceConfigOverrideCreateCall), [*environments trace config overrides delete*](api::OrganizationEnvironmentTraceConfigOverrideDeleteCall), [*environments trace config overrides get*](api::OrganizationEnvironmentTraceConfigOverrideGetCall), [*environments trace config overrides list*](api::OrganizationEnvironmentTraceConfigOverrideListCall), [*environments trace config overrides patch*](api::OrganizationEnvironmentTraceConfigOverridePatchCall), [*environments unsubscribe*](api::OrganizationEnvironmentUnsubscribeCall), [*environments update*](api::OrganizationEnvironmentUpdateCall), [*environments update debugmask*](api::OrganizationEnvironmentUpdateDebugmaskCall), [*environments update environment*](api::OrganizationEnvironmentUpdateEnvironmentCall), [*environments update trace config*](api::OrganizationEnvironmentUpdateTraceConfigCall), [*get*](api::OrganizationGetCall), [*get deployed ingress config*](api::OrganizationGetDeployedIngressConfigCall), [*get sync authorization*](api::OrganizationGetSyncAuthorizationCall), [*host queries create*](api::OrganizationHostQueryCreateCall), [*host queries get*](api::OrganizationHostQueryGetCall), [*host queries get result*](api::OrganizationHostQueryGetResultCall), [*host queries get result view*](api::OrganizationHostQueryGetResultViewCall), [*host queries list*](api::OrganizationHostQueryListCall), [*host stats get*](api::OrganizationHostStatGetCall), [*instances attachments create*](api::OrganizationInstanceAttachmentCreateCall), [*instances attachments delete*](api::OrganizationInstanceAttachmentDeleteCall), [*instances attachments get*](api::OrganizationInstanceAttachmentGetCall), [*instances attachments list*](api::OrganizationInstanceAttachmentListCall), [*instances canaryevaluations create*](api::OrganizationInstanceCanaryevaluationCreateCall), [*instances canaryevaluations get*](api::OrganizationInstanceCanaryevaluationGetCall), [*instances create*](api::OrganizationInstanceCreateCall), [*instances delete*](api::OrganizationInstanceDeleteCall), [*instances get*](api::OrganizationInstanceGetCall), [*instances list*](api::OrganizationInstanceListCall), [*instances nat addresses activate*](api::OrganizationInstanceNatAddresseActivateCall), [*instances nat addresses create*](api::OrganizationInstanceNatAddresseCreateCall), [*instances nat addresses delete*](api::OrganizationInstanceNatAddresseDeleteCall), [*instances nat addresses get*](api::OrganizationInstanceNatAddresseGetCall), [*instances nat addresses list*](api::OrganizationInstanceNatAddresseListCall), [*instances report status*](api::OrganizationInstanceReportStatuCall), [*keyvaluemaps create*](api::OrganizationKeyvaluemapCreateCall), [*keyvaluemaps delete*](api::OrganizationKeyvaluemapDeleteCall), [*list*](api::OrganizationListCall), [*operations get*](api::OrganizationOperationGetCall), [*operations list*](api::OrganizationOperationListCall), [*optimized host stats get*](api::OrganizationOptimizedHostStatGetCall), [*reports create*](api::OrganizationReportCreateCall), [*reports delete*](api::OrganizationReportDeleteCall), [*reports get*](api::OrganizationReportGetCall), [*reports list*](api::OrganizationReportListCall), [*reports update*](api::OrganizationReportUpdateCall), [*set sync authorization*](api::OrganizationSetSyncAuthorizationCall), [*sharedflows create*](api::OrganizationSharedflowCreateCall), [*sharedflows delete*](api::OrganizationSharedflowDeleteCall), [*sharedflows deployments list*](api::OrganizationSharedflowDeploymentListCall), [*sharedflows get*](api::OrganizationSharedflowGetCall), [*sharedflows list*](api::OrganizationSharedflowListCall), [*sharedflows revisions delete*](api::OrganizationSharedflowRevisionDeleteCall), [*sharedflows revisions deployments list*](api::OrganizationSharedflowRevisionDeploymentListCall), [*sharedflows revisions get*](api::OrganizationSharedflowRevisionGetCall), [*sharedflows revisions update shared flow revision*](api::OrganizationSharedflowRevisionUpdateSharedFlowRevisionCall), [*sites apicategories create*](api::OrganizationSiteApicategoryCreateCall), [*sites apicategories delete*](api::OrganizationSiteApicategoryDeleteCall), [*sites apicategories get*](api::OrganizationSiteApicategoryGetCall), [*sites apicategories list*](api::OrganizationSiteApicategoryListCall), [*sites apicategories patch*](api::OrganizationSiteApicategoryPatchCall) and [*update*](api::OrganizationUpdateCall)
//! * projects
//!  * [*provision organization*](api::ProjectProvisionOrganizationCall)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Apigee)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.organizations().envgroups_attachments_create(...).doit().await
//! let r = hub.organizations().envgroups_attachments_delete(...).doit().await
//! let r = hub.organizations().envgroups_create(...).doit().await
//! let r = hub.organizations().envgroups_delete(...).doit().await
//! let r = hub.organizations().envgroups_patch(...).doit().await
//! let r = hub.organizations().environments_create(...).doit().await
//! let r = hub.organizations().environments_delete(...).doit().await
//! let r = hub.organizations().instances_attachments_create(...).doit().await
//! let r = hub.organizations().instances_attachments_delete(...).doit().await
//! let r = hub.organizations().instances_canaryevaluations_create(...).doit().await
//! let r = hub.organizations().instances_nat_addresses_activate(...).doit().await
//! let r = hub.organizations().instances_nat_addresses_create(...).doit().await
//! let r = hub.organizations().instances_nat_addresses_delete(...).doit().await
//! let r = hub.organizations().instances_create(...).doit().await
//! let r = hub.organizations().instances_delete(...).doit().await
//! let r = hub.organizations().operations_get(...).doit().await
//! let r = hub.organizations().create(...).doit().await
//! let r = hub.organizations().delete(...).doit().await
//! let r = hub.projects().provision_organization(...).doit().await
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-apigee1 = "*"
//! hyper = "^0.14"
//! hyper-rustls = "^0.22"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^5.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_apigee1 as apigee1;
//! use apigee1::api::GoogleCloudApigeeV1EnvironmentGroup;
//! use apigee1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use oauth2;
//! use apigee1::Apigee;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = Apigee::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GoogleCloudApigeeV1EnvironmentGroup::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.organizations().envgroups_create(req, "parent")
//!              .name("ipsum")
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

pub mod api;
pub mod client;

// Re-export the hub type and some basic client structs
pub use api::Apigee;
pub use client::{Result, Error, Delegate};
