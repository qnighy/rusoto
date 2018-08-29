// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use std::io::Write;
use std::str::FromStr;
use xml;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;
use xml::EventWriter;
enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
/// <p>Specifies the days since the initiation of an Incomplete Multipart Upload that Lifecycle will wait before permanently removing all parts of the upload.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortIncompleteMultipartUpload {
    /// <p>Indicates the number of days that must pass since initiation for Lifecycle to abort an Incomplete Multipart Upload.</p>
    pub days_after_initiation: Option<i64>,
}

struct AbortIncompleteMultipartUploadDeserializer;
impl AbortIncompleteMultipartUploadDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AbortIncompleteMultipartUpload, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AbortIncompleteMultipartUpload::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DaysAfterInitiation" => {
                        obj.days_after_initiation =
                            Some(try!(DaysAfterInitiationDeserializer::deserialize(
                                "DaysAfterInitiation",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AbortIncompleteMultipartUploadSerializer;
impl AbortIncompleteMultipartUploadSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AbortIncompleteMultipartUpload,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.days_after_initiation {
            writer.write(xml::writer::XmlEvent::start_element("DaysAfterInitiation"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<String>,
}

struct AbortMultipartUploadOutputDeserializer;
impl AbortMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AbortMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = AbortMultipartUploadOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortMultipartUploadRequest {
    pub bucket: String,
    pub key: String,
    pub request_payer: Option<String>,
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccelerateConfiguration {
    /// <p>The accelerate configuration of the bucket.</p>
    pub status: Option<String>,
}

pub struct AccelerateConfigurationSerializer;
impl AccelerateConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AccelerateConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.status {
            writer.write(xml::writer::XmlEvent::start_element("Status"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessControlPolicy {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    pub owner: Option<Owner>,
}

pub struct AccessControlPolicySerializer;
impl AccessControlPolicySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AccessControlPolicy,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.grants {
            &GrantsSerializer::serialize(&mut writer, "AccessControlList", value)?;
        }
        if let Some(ref value) = obj.owner {
            &OwnerSerializer::serialize(&mut writer, "Owner", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for information regarding the access control for replicas.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessControlTranslation {
    /// <p>The override value for the owner of the replica object.</p>
    pub owner: String,
}

struct AccessControlTranslationDeserializer;
impl AccessControlTranslationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccessControlTranslation, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AccessControlTranslation::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Owner" => {
                        obj.owner = try!(OwnerOverrideDeserializer::deserialize("Owner", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AccessControlTranslationSerializer;
impl AccessControlTranslationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AccessControlTranslation,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Owner"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.owner
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AccountIdDeserializer;
impl AccountIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AccountIdSerializer;
impl AccountIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct AllowQuotedRecordDelimiterSerializer;
impl AllowQuotedRecordDelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AllowedHeaderDeserializer;
impl AllowedHeaderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AllowedHeaderSerializer;
impl AllowedHeaderSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AllowedHeadersDeserializer;
impl AllowedHeadersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AllowedHeaderDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct AllowedHeadersSerializer;
impl AllowedHeadersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            AllowedHeaderSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct AllowedMethodDeserializer;
impl AllowedMethodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AllowedMethodSerializer;
impl AllowedMethodSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AllowedMethodsDeserializer;
impl AllowedMethodsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AllowedMethodDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct AllowedMethodsSerializer;
impl AllowedMethodsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            AllowedMethodSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct AllowedOriginDeserializer;
impl AllowedOriginDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AllowedOriginSerializer;
impl AllowedOriginSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AllowedOriginsDeserializer;
impl AllowedOriginsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AllowedOriginDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct AllowedOriginsSerializer;
impl AllowedOriginsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            AllowedOriginSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsAndOperator {
    /// <p>The prefix to use when evaluating an AND predicate.</p>
    pub prefix: Option<String>,
    /// <p>The list of tags to use when evaluating an AND predicate.</p>
    pub tags: Option<Vec<Tag>>,
}

struct AnalyticsAndOperatorDeserializer;
impl AnalyticsAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsAndOperator, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsAndOperator::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "Tag" => {
                        obj.tags = Some(try!(TagSetDeserializer::deserialize("Tag", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AnalyticsAndOperatorSerializer;
impl AnalyticsAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsAndOperator,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsConfiguration {
    /// <p>The filter used to describe a set of objects for analyses. A filter must have exactly one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided, all objects will be considered in any analysis.</p>
    pub filter: Option<AnalyticsFilter>,
    /// <p>The identifier used to represent an analytics configuration.</p>
    pub id: String,
    /// <p>If present, it indicates that data related to access patterns will be collected and made available to analyze the tradeoffs between different storage classes.</p>
    pub storage_class_analysis: StorageClassAnalysis,
}

struct AnalyticsConfigurationDeserializer;
impl AnalyticsConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Filter" => {
                        obj.filter = Some(try!(AnalyticsFilterDeserializer::deserialize(
                            "Filter", stack
                        )));
                    }
                    "Id" => {
                        obj.id = try!(AnalyticsIdDeserializer::deserialize("Id", stack));
                    }
                    "StorageClassAnalysis" => {
                        obj.storage_class_analysis =
                            try!(StorageClassAnalysisDeserializer::deserialize(
                                "StorageClassAnalysis",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AnalyticsConfigurationSerializer;
impl AnalyticsConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter {
            &AnalyticsFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        StorageClassAnalysisSerializer::serialize(
            &mut writer,
            "StorageClassAnalysis",
            &obj.storage_class_analysis,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AnalyticsConfigurationListDeserializer;
impl AnalyticsConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AnalyticsConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AnalyticsConfigurationDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsExportDestination {
    /// <p>A destination signifying output to an S3 bucket.</p>
    pub s3_bucket_destination: AnalyticsS3BucketDestination,
}

struct AnalyticsExportDestinationDeserializer;
impl AnalyticsExportDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsExportDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsExportDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "S3BucketDestination" => {
                        obj.s3_bucket_destination =
                            try!(AnalyticsS3BucketDestinationDeserializer::deserialize(
                                "S3BucketDestination",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AnalyticsExportDestinationSerializer;
impl AnalyticsExportDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsExportDestination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        AnalyticsS3BucketDestinationSerializer::serialize(
            &mut writer,
            "S3BucketDestination",
            &obj.s3_bucket_destination,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsFilter {
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating an analytics filter. The operator must have at least two predicates.</p>
    pub and: Option<AnalyticsAndOperator>,
    /// <p>The prefix to use when evaluating an analytics filter.</p>
    pub prefix: Option<String>,
    /// <p>The tag to use when evaluating an analytics filter.</p>
    pub tag: Option<Tag>,
}

struct AnalyticsFilterDeserializer;
impl AnalyticsFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "And" => {
                        obj.and = Some(try!(AnalyticsAndOperatorDeserializer::deserialize(
                            "And", stack
                        )));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "Tag" => {
                        obj.tag = Some(try!(TagDeserializer::deserialize("Tag", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AnalyticsFilterSerializer;
impl AnalyticsFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &AnalyticsAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tag {
            &TagSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AnalyticsIdDeserializer;
impl AnalyticsIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AnalyticsIdSerializer;
impl AnalyticsIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsS3BucketDestination {
    /// <p>The Amazon resource name (ARN) of the bucket to which data is exported.</p>
    pub bucket: String,
    /// <p>The account ID that owns the destination bucket. If no account ID is provided, the owner will not be validated prior to exporting data.</p>
    pub bucket_account_id: Option<String>,
    /// <p>The file format used when exporting data to Amazon S3.</p>
    pub format: String,
    /// <p>The prefix to use when exporting data. The exported data begins with this prefix.</p>
    pub prefix: Option<String>,
}

struct AnalyticsS3BucketDestinationDeserializer;
impl AnalyticsS3BucketDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsS3BucketDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsS3BucketDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Bucket" => {
                        obj.bucket = try!(BucketNameDeserializer::deserialize("Bucket", stack));
                    }
                    "BucketAccountId" => {
                        obj.bucket_account_id = Some(try!(AccountIdDeserializer::deserialize(
                            "BucketAccountId",
                            stack
                        )));
                    }
                    "Format" => {
                        obj.format = try!(AnalyticsS3ExportFileFormatDeserializer::deserialize(
                            "Format", stack
                        ));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AnalyticsS3BucketDestinationSerializer;
impl AnalyticsS3BucketDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsS3BucketDestination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.bucket_account_id {
            writer.write(xml::writer::XmlEvent::start_element("BucketAccountId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Format"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.format
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AnalyticsS3ExportFileFormatDeserializer;
impl AnalyticsS3ExportFileFormatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct AnalyticsS3ExportFileFormatSerializer;
impl AnalyticsS3ExportFileFormatSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct StreamingBody {
    len: Option<usize>,
    inner: Box<::futures::Stream<Item = Vec<u8>, Error = ::std::io::Error> + Send>,
}

impl StreamingBody {
    pub fn new<S>(stream: S) -> StreamingBody
    where
        S: ::futures::Stream<Item = Vec<u8>, Error = ::std::io::Error> + Send + 'static,
    {
        StreamingBody {
            len: None,
            inner: Box::new(stream),
        }
    }
}

impl From<Vec<u8>> for StreamingBody {
    fn from(buf: Vec<u8>) -> StreamingBody {
        StreamingBody {
            len: Some(buf.len()),
            inner: Box::new(::futures::stream::once(Ok(buf))),
        }
    }
}

impl fmt::Debug for StreamingBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Body: streaming content, len = {:?}>", self.len)
    }
}

impl ::futures::Stream for StreamingBody {
    type Item = Vec<u8>;
    type Error = ::std::io::Error;

    fn poll(&mut self) -> ::futures::Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}
struct BodyDeserializer;
impl BodyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<u8>, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack)).into_bytes();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct BodySerializer;
impl BodySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<u8>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = String::from_utf8(obj.to_vec()).expect("Not a UTF-8 string")
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bucket {
    /// <p>Date the bucket was created.</p>
    pub creation_date: Option<String>,
    /// <p>The name of the bucket.</p>
    pub name: Option<String>,
}

struct BucketDeserializer;
impl BucketDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Bucket, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Bucket::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CreationDate" => {
                        obj.creation_date = Some(try!(CreationDateDeserializer::deserialize(
                            "CreationDate",
                            stack
                        )));
                    }
                    "Name" => {
                        obj.name = Some(try!(BucketNameDeserializer::deserialize("Name", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BucketAccelerateStatusDeserializer;
impl BucketAccelerateStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct BucketAccelerateStatusSerializer;
impl BucketAccelerateStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BucketLifecycleConfiguration {
    pub rules: Vec<LifecycleRule>,
}

pub struct BucketLifecycleConfigurationSerializer;
impl BucketLifecycleConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &BucketLifecycleConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        LifecycleRulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketLocationConstraintDeserializer;
impl BucketLocationConstraintDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct BucketLocationConstraintSerializer;
impl BucketLocationConstraintSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}

pub struct BucketLoggingStatusSerializer;
impl BucketLoggingStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &BucketLoggingStatus,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.logging_enabled {
            &LoggingEnabledSerializer::serialize(&mut writer, "LoggingEnabled", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketLogsPermissionDeserializer;
impl BucketLogsPermissionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct BucketLogsPermissionSerializer;
impl BucketLogsPermissionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketNameDeserializer;
impl BucketNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct BucketNameSerializer;
impl BucketNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketVersioningStatusDeserializer;
impl BucketVersioningStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct BucketVersioningStatusSerializer;
impl BucketVersioningStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketsDeserializer;
impl BucketsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Bucket>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Bucket" {
                        obj.push(try!(BucketDeserializer::deserialize("Bucket", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct BytesProcessedDeserializer;
impl BytesProcessedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BytesReturnedDeserializer;
impl BytesReturnedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BytesScannedDeserializer;
impl BytesScannedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CORSConfiguration {
    pub cors_rules: Vec<CORSRule>,
}

pub struct CORSConfigurationSerializer;
impl CORSConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CORSConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        CORSRulesSerializer::serialize(&mut writer, "CORSRule", &obj.cors_rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CORSRule {
    /// <p>Specifies which headers are allowed in a pre-flight OPTIONS request.</p>
    pub allowed_headers: Option<Vec<String>>,
    /// <p>Identifies HTTP methods that the domain/origin specified in the rule is allowed to execute.</p>
    pub allowed_methods: Vec<String>,
    /// <p>One or more origins you want customers to be able to access the bucket from.</p>
    pub allowed_origins: Vec<String>,
    /// <p>One or more headers in the response that you want customers to be able to access from their applications (for example, from a JavaScript XMLHttpRequest object).</p>
    pub expose_headers: Option<Vec<String>>,
    /// <p>The time in seconds that your browser is to cache the preflight response for the specified resource.</p>
    pub max_age_seconds: Option<i64>,
}

struct CORSRuleDeserializer;
impl CORSRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CORSRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CORSRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AllowedHeader" => {
                            obj.allowed_headers = Some(try!(
                                AllowedHeadersDeserializer::deserialize("AllowedHeader", stack)
                            ));
                        }
                        "AllowedMethod" => {
                            obj.allowed_methods = try!(AllowedMethodsDeserializer::deserialize(
                                "AllowedMethod",
                                stack
                            ));
                        }
                        "AllowedOrigin" => {
                            obj.allowed_origins = try!(AllowedOriginsDeserializer::deserialize(
                                "AllowedOrigin",
                                stack
                            ));
                        }
                        "ExposeHeader" => {
                            obj.expose_headers = Some(try!(
                                ExposeHeadersDeserializer::deserialize("ExposeHeader", stack)
                            ));
                        }
                        "MaxAgeSeconds" => {
                            obj.max_age_seconds = Some(try!(
                                MaxAgeSecondsDeserializer::deserialize("MaxAgeSeconds", stack)
                            ));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct CORSRuleSerializer;
impl CORSRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CORSRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.allowed_headers {
            &AllowedHeadersSerializer::serialize(&mut writer, "AllowedHeader", value)?;
        }
        AllowedMethodsSerializer::serialize(&mut writer, "AllowedMethod", &obj.allowed_methods)?;
        AllowedOriginsSerializer::serialize(&mut writer, "AllowedOrigin", &obj.allowed_origins)?;
        if let Some(ref value) = obj.expose_headers {
            &ExposeHeadersSerializer::serialize(&mut writer, "ExposeHeader", value)?;
        }
        if let Some(ref value) = obj.max_age_seconds {
            writer.write(xml::writer::XmlEvent::start_element("MaxAgeSeconds"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CORSRulesDeserializer;
impl CORSRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CORSRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(CORSRuleDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct CORSRulesSerializer;
impl CORSRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<CORSRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            CORSRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p>Describes how a CSV-formatted input object is formatted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CSVInput {
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records should be allowed. Default value is FALSE. Setting this value to TRUE may lower performance.</p>
    pub allow_quoted_record_delimiter: Option<bool>,
    /// <p>Single character used to indicate a row should be ignored when present at the start of a row.</p>
    pub comments: Option<String>,
    /// <p>Value used to separate individual fields in a record.</p>
    pub field_delimiter: Option<String>,
    /// <p>Describes the first line of input. Valid values: None, Ignore, Use.</p>
    pub file_header_info: Option<String>,
    /// <p>Value used for escaping where the field delimiter is part of the value.</p>
    pub quote_character: Option<String>,
    /// <p>Single character used for escaping the quote character inside an already escaped value.</p>
    pub quote_escape_character: Option<String>,
    /// <p>Value used to separate individual records.</p>
    pub record_delimiter: Option<String>,
}

pub struct CSVInputSerializer;
impl CSVInputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CSVInput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.allow_quoted_record_delimiter {
            writer.write(xml::writer::XmlEvent::start_element(
                "AllowQuotedRecordDelimiter",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.comments {
            writer.write(xml::writer::XmlEvent::start_element("Comments"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.field_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("FieldDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.file_header_info {
            writer.write(xml::writer::XmlEvent::start_element("FileHeaderInfo"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_character {
            writer.write(xml::writer::XmlEvent::start_element("QuoteCharacter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_escape_character {
            writer.write(xml::writer::XmlEvent::start_element("QuoteEscapeCharacter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.record_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("RecordDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes how CSV-formatted results are formatted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CSVOutput {
    /// <p>Value used to separate individual fields in a record.</p>
    pub field_delimiter: Option<String>,
    /// <p>Value used for escaping where the field delimiter is part of the value.</p>
    pub quote_character: Option<String>,
    /// <p>Single character used for escaping the quote character inside an already escaped value.</p>
    pub quote_escape_character: Option<String>,
    /// <p>Indicates whether or not all output fields should be quoted.</p>
    pub quote_fields: Option<String>,
    /// <p>Value used to separate individual records.</p>
    pub record_delimiter: Option<String>,
}

pub struct CSVOutputSerializer;
impl CSVOutputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CSVOutput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.field_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("FieldDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_character {
            writer.write(xml::writer::XmlEvent::start_element("QuoteCharacter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_escape_character {
            writer.write(xml::writer::XmlEvent::start_element("QuoteEscapeCharacter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_fields {
            writer.write(xml::writer::XmlEvent::start_element("QuoteFields"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.record_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("RecordDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CloudFunctionDeserializer;
impl CloudFunctionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct CloudFunctionSerializer;
impl CloudFunctionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CloudFunctionConfiguration {
    pub cloud_function: Option<String>,
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    pub invocation_role: Option<String>,
}

struct CloudFunctionConfigurationDeserializer;
impl CloudFunctionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFunctionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudFunctionConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFunction" => {
                        obj.cloud_function = Some(try!(CloudFunctionDeserializer::deserialize(
                            "CloudFunction",
                            stack
                        )));
                    }
                    "Event" => {
                        obj.events = Some(try!(EventListDeserializer::deserialize("Event", stack)));
                    }
                    "Id" => {
                        obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id", stack)));
                    }
                    "InvocationRole" => {
                        obj.invocation_role =
                            Some(try!(CloudFunctionInvocationRoleDeserializer::deserialize(
                                "InvocationRole",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct CloudFunctionConfigurationSerializer;
impl CloudFunctionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CloudFunctionConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.cloud_function {
            writer.write(xml::writer::XmlEvent::start_element("CloudFunction"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.invocation_role {
            writer.write(xml::writer::XmlEvent::start_element("InvocationRole"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CloudFunctionInvocationRoleDeserializer;
impl CloudFunctionInvocationRoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct CloudFunctionInvocationRoleSerializer;
impl CloudFunctionInvocationRoleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CodeDeserializer;
impl CodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct CommentsSerializer;
impl CommentsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CommonPrefix {
    pub prefix: Option<String>,
}

struct CommonPrefixDeserializer;
impl CommonPrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CommonPrefix, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CommonPrefix::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct CommonPrefixListDeserializer;
impl CommonPrefixListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CommonPrefix>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(CommonPrefixDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompleteMultipartUploadOutput {
    pub bucket: Option<String>,
    /// <p>Entity tag of the object.</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured, this will contain the expiration date (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    pub key: Option<String>,
    pub location: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
}

struct CompleteMultipartUploadOutputDeserializer;
impl CompleteMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CompleteMultipartUploadOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Bucket" => {
                        obj.bucket =
                            Some(try!(BucketNameDeserializer::deserialize("Bucket", stack)));
                    }
                    "ETag" => {
                        obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "Location" => {
                        obj.location =
                            Some(try!(LocationDeserializer::deserialize("Location", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompleteMultipartUploadRequest {
    pub bucket: String,
    pub key: String,
    pub multipart_upload: Option<CompletedMultipartUpload>,
    pub request_payer: Option<String>,
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompletedMultipartUpload {
    pub parts: Option<Vec<CompletedPart>>,
}

pub struct CompletedMultipartUploadSerializer;
impl CompletedMultipartUploadSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CompletedMultipartUpload,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.parts {
            &CompletedPartListSerializer::serialize(&mut writer, "Part", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompletedPart {
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub e_tag: Option<String>,
    /// <p>Part number that identifies the part. This is a positive integer between 1 and 10,000.</p>
    pub part_number: Option<i64>,
}

pub struct CompletedPartSerializer;
impl CompletedPartSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CompletedPart,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.e_tag {
            writer.write(xml::writer::XmlEvent::start_element("ETag"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.part_number {
            writer.write(xml::writer::XmlEvent::start_element("PartNumber"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct CompletedPartListSerializer;
impl CompletedPartListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<CompletedPart>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            CompletedPartSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

pub struct CompressionTypeSerializer;
impl CompressionTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Condition {
    /// <p>The HTTP error code when the redirect is applied. In the event of an error, if the error code equals this value, then the specified redirect is applied. Required when parent element Condition is specified and sibling KeyPrefixEquals is not specified. If both are specified, then both must be true for the redirect to be applied.</p>
    pub http_error_code_returned_equals: Option<String>,
    /// <p>The object key name prefix when the redirect is applied. For example, to redirect requests for ExamplePage.html, the key prefix will be ExamplePage.html. To redirect request for all pages with the prefix docs/, the key prefix will be /docs, which identifies all objects in the docs/ folder. Required when the parent element Condition is specified and sibling HttpErrorCodeReturnedEquals is not specified. If both conditions are specified, both must be true for the redirect to be applied.</p>
    pub key_prefix_equals: Option<String>,
}

struct ConditionDeserializer;
impl ConditionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Condition, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Condition::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HttpErrorCodeReturnedEquals" => {
                        obj.http_error_code_returned_equals =
                            Some(try!(HttpErrorCodeReturnedEqualsDeserializer::deserialize(
                                "HttpErrorCodeReturnedEquals",
                                stack
                            )));
                    }
                    "KeyPrefixEquals" => {
                        obj.key_prefix_equals = Some(try!(
                            KeyPrefixEqualsDeserializer::deserialize("KeyPrefixEquals", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ConditionSerializer;
impl ConditionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Condition,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.http_error_code_returned_equals {
            writer.write(xml::writer::XmlEvent::start_element(
                "HttpErrorCodeReturnedEquals",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.key_prefix_equals {
            writer.write(xml::writer::XmlEvent::start_element("KeyPrefixEquals"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContinuationEvent {}

struct ContinuationEventDeserializer;
impl ContinuationEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ContinuationEvent, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = ContinuationEvent::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyObjectOutput {
    pub copy_object_result: Option<CopyObjectResult>,
    pub copy_source_version_id: Option<String>,
    /// <p>If the object expiration is configured, the response includes this header.</p>
    pub expiration: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>Version ID of the newly created copy.</p>
    pub version_id: Option<String>,
}

struct CopyObjectOutputDeserializer;
impl CopyObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyObjectOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CopyObjectResult" => {
                        obj.copy_object_result = Some(try!(
                            CopyObjectResultDeserializer::deserialize("CopyObjectResult", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyObjectRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    pub bucket: String,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>The name of the source bucket and key name of the source object, separated by a slash (/). Must be URL-encoded.</p>
    pub copy_source: String,
    /// <p>Copies the object if its entity tag (ETag) matches the specified tag.</p>
    pub copy_source_if_match: Option<String>,
    /// <p>Copies the object if it has been modified since the specified time.</p>
    pub copy_source_if_modified_since: Option<String>,
    /// <p>Copies the object if its entity tag (ETag) is different than the specified ETag.</p>
    pub copy_source_if_none_match: Option<String>,
    /// <p>Copies the object if it hasn't been modified since the specified time.</p>
    pub copy_source_if_unmodified_since: Option<String>,
    /// <p>Specifies the algorithm to use when decrypting the source object (e.g., AES256).</p>
    pub copy_source_sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.</p>
    pub copy_source_sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub copy_source_sse_customer_key_md5: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to read the object data and its metadata.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the object ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable object.</p>
    pub grant_write_acp: Option<String>,
    pub key: String,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether the metadata is copied from the source object or replaced with metadata provided in the request.</p>
    pub metadata_directive: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>The type of storage to use for the object. Defaults to 'STANDARD'.</p>
    pub storage_class: Option<String>,
    /// <p>The tag-set for the object destination object this value must be used in conjunction with the TaggingDirective. The tag-set must be encoded as URL Query parameters</p>
    pub tagging: Option<String>,
    /// <p>Specifies whether the object tag-set are copied from the source object or replaced with tag-set provided in the request.</p>
    pub tagging_directive: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyObjectResult {
    pub e_tag: Option<String>,
    pub last_modified: Option<String>,
}

struct CopyObjectResultDeserializer;
impl CopyObjectResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyObjectResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyObjectResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ETag" => {
                        obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                    }
                    "LastModified" => {
                        obj.last_modified = Some(try!(LastModifiedDeserializer::deserialize(
                            "LastModified",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyPartResult {
    /// <p>Entity tag of the object.</p>
    pub e_tag: Option<String>,
    /// <p>Date and time at which the object was uploaded.</p>
    pub last_modified: Option<String>,
}

struct CopyPartResultDeserializer;
impl CopyPartResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyPartResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyPartResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ETag" => {
                        obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                    }
                    "LastModified" => {
                        obj.last_modified = Some(try!(LastModifiedDeserializer::deserialize(
                            "LastModified",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateBucketConfiguration {
    /// <p>Specifies the region where the bucket will be created. If you don't specify a region, the bucket will be created in US Standard.</p>
    pub location_constraint: Option<String>,
}

pub struct CreateBucketConfigurationSerializer;
impl CreateBucketConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateBucketConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.location_constraint {
            writer.write(xml::writer::XmlEvent::start_element("LocationConstraint"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateBucketOutput {
    pub location: Option<String>,
}

struct CreateBucketOutputDeserializer;
impl CreateBucketOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateBucketOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateBucketOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateBucketRequest {
    /// <p>The canned ACL to apply to the bucket.</p>
    pub acl: Option<String>,
    pub bucket: String,
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to list the objects in the bucket.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to create, overwrite, and delete any object in the bucket.</p>
    pub grant_write: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    pub grant_write_acp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateMultipartUploadOutput {
    /// <p>Date when multipart upload will become eligible for abort operation by lifecycle.</p>
    pub abort_date: Option<String>,
    /// <p>Id of the lifecycle rule that makes a multipart upload eligible for abort operation.</p>
    pub abort_rule_id: Option<String>,
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: Option<String>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>ID for the initiated multipart upload.</p>
    pub upload_id: Option<String>,
}

struct CreateMultipartUploadOutputDeserializer;
impl CreateMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateMultipartUploadOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Bucket" => {
                        obj.bucket =
                            Some(try!(BucketNameDeserializer::deserialize("Bucket", stack)));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "UploadId" => {
                        obj.upload_id = Some(try!(MultipartUploadIdDeserializer::deserialize(
                            "UploadId", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateMultipartUploadRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    pub bucket: String,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to read the object data and its metadata.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the object ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable object.</p>
    pub grant_write_acp: Option<String>,
    pub key: String,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>The type of storage to use for the object. Defaults to 'STANDARD'.</p>
    pub storage_class: Option<String>,
    /// <p>The tag-set for the object. The tag-set must be encoded as URL Query parameters</p>
    pub tagging: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

struct CreationDateDeserializer;
impl CreationDateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DateDeserializer;
impl DateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct DateSerializer;
impl DateSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DaysDeserializer;
impl DaysDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct DaysSerializer;
impl DaysSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DaysAfterInitiationDeserializer;
impl DaysAfterInitiationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct DaysAfterInitiationSerializer;
impl DaysAfterInitiationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Delete {
    pub objects: Vec<ObjectIdentifier>,
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set its value to true.</p>
    pub quiet: Option<bool>,
}

pub struct DeleteSerializer;
impl DeleteSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Delete,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        ObjectIdentifierListSerializer::serialize(&mut writer, "Object", &obj.objects)?;
        if let Some(ref value) = obj.quiet {
            writer.write(xml::writer::XmlEvent::start_element("Quiet"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub bucket: String,
    /// <p>The identifier used to represent an analytics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketCorsRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketEncryptionRequest {
    /// <p>The name of the bucket containing the server-side encryption configuration to delete.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketInventoryConfigurationRequest {
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    pub bucket: String,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketLifecycleRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketMetricsConfigurationRequest {
    /// <p>The name of the bucket containing the metrics configuration to delete.</p>
    pub bucket: String,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketPolicyRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketReplicationRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketTaggingRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketWebsiteRequest {
    pub bucket: String,
}

struct DeleteMarkerDeserializer;
impl DeleteMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteMarkerEntry {
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub is_latest: Option<bool>,
    /// <p>The object key.</p>
    pub key: Option<String>,
    /// <p>Date and time the object was last modified.</p>
    pub last_modified: Option<String>,
    pub owner: Option<Owner>,
    /// <p>Version ID of an object.</p>
    pub version_id: Option<String>,
}

struct DeleteMarkerEntryDeserializer;
impl DeleteMarkerEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteMarkerEntry, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteMarkerEntry::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IsLatest" => {
                        obj.is_latest =
                            Some(try!(IsLatestDeserializer::deserialize("IsLatest", stack)));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "LastModified" => {
                        obj.last_modified = Some(try!(LastModifiedDeserializer::deserialize(
                            "LastModified",
                            stack
                        )));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                    }
                    "VersionId" => {
                        obj.version_id = Some(try!(ObjectVersionIdDeserializer::deserialize(
                            "VersionId",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DeleteMarkerVersionIdDeserializer;
impl DeleteMarkerVersionIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DeleteMarkersDeserializer;
impl DeleteMarkersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DeleteMarkerEntry>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(DeleteMarkerEntryDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectOutput {
    /// <p>Specifies whether the versioned object that was permanently deleted was (true) or was not (false) a delete marker.</p>
    pub delete_marker: Option<bool>,
    pub request_charged: Option<String>,
    /// <p>Returns the version ID of the delete marker created as a result of the DELETE operation.</p>
    pub version_id: Option<String>,
}

struct DeleteObjectOutputDeserializer;
impl DeleteObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteObjectOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectRequest {
    pub bucket: String,
    pub key: String,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub mfa: Option<String>,
    pub request_payer: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectTaggingOutput {
    /// <p>The versionId of the object the tag-set was removed from.</p>
    pub version_id: Option<String>,
}

struct DeleteObjectTaggingOutputDeserializer;
impl DeleteObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteObjectTaggingOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectTaggingRequest {
    pub bucket: String,
    pub key: String,
    /// <p>The versionId of the object that the tag-set will be removed from.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectsOutput {
    pub deleted: Option<Vec<DeletedObject>>,
    pub errors: Option<Vec<S3Error>>,
    pub request_charged: Option<String>,
}

struct DeleteObjectsOutputDeserializer;
impl DeleteObjectsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteObjectsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Deleted" => {
                        obj.deleted = Some(try!(DeletedObjectsDeserializer::deserialize(
                            "Deleted", stack
                        )));
                    }
                    "Error" => {
                        obj.errors = Some(try!(ErrorsDeserializer::deserialize("Error", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectsRequest {
    pub bucket: String,
    pub delete: Delete,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub mfa: Option<String>,
    pub request_payer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletedObject {
    pub delete_marker: Option<bool>,
    pub delete_marker_version_id: Option<String>,
    pub key: Option<String>,
    pub version_id: Option<String>,
}

struct DeletedObjectDeserializer;
impl DeletedObjectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeletedObject, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeletedObject::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DeleteMarker" => {
                        obj.delete_marker = Some(try!(DeleteMarkerDeserializer::deserialize(
                            "DeleteMarker",
                            stack
                        )));
                    }
                    "DeleteMarkerVersionId" => {
                        obj.delete_marker_version_id =
                            Some(try!(DeleteMarkerVersionIdDeserializer::deserialize(
                                "DeleteMarkerVersionId",
                                stack
                            )));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "VersionId" => {
                        obj.version_id = Some(try!(ObjectVersionIdDeserializer::deserialize(
                            "VersionId",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DeletedObjectsDeserializer;
impl DeletedObjectsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DeletedObject>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(DeletedObjectDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
struct DelimiterDeserializer;
impl DelimiterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct DelimiterSerializer;
impl DelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct DescriptionSerializer;
impl DescriptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for replication destination information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Destination {
    /// <p>Container for information regarding the access control for replicas.</p>
    pub access_control_translation: Option<AccessControlTranslation>,
    /// <p>Account ID of the destination bucket. Currently this is only being verified if Access Control Translation is enabled</p>
    pub account: Option<String>,
    /// <p>Amazon resource name (ARN) of the bucket where you want Amazon S3 to store replicas of the object identified by the rule.</p>
    pub bucket: String,
    /// <p>Container for information regarding encryption based configuration for replicas.</p>
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
}

struct DestinationDeserializer;
impl DestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Destination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Destination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AccessControlTranslation" => {
                        obj.access_control_translation =
                            Some(try!(AccessControlTranslationDeserializer::deserialize(
                                "AccessControlTranslation",
                                stack
                            )));
                    }
                    "Account" => {
                        obj.account =
                            Some(try!(AccountIdDeserializer::deserialize("Account", stack)));
                    }
                    "Bucket" => {
                        obj.bucket = try!(BucketNameDeserializer::deserialize("Bucket", stack));
                    }
                    "EncryptionConfiguration" => {
                        obj.encryption_configuration =
                            Some(try!(EncryptionConfigurationDeserializer::deserialize(
                                "EncryptionConfiguration",
                                stack
                            )));
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(try!(StorageClassDeserializer::deserialize(
                            "StorageClass",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct DestinationSerializer;
impl DestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Destination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.access_control_translation {
            &AccessControlTranslationSerializer::serialize(
                &mut writer,
                "AccessControlTranslation",
                value,
            )?;
        }
        if let Some(ref value) = obj.account {
            writer.write(xml::writer::XmlEvent::start_element("Account"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.encryption_configuration {
            &EncryptionConfigurationSerializer::serialize(
                &mut writer,
                "EncryptionConfiguration",
                value,
            )?;
        }
        if let Some(ref value) = obj.storage_class {
            writer.write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DisplayNameDeserializer;
impl DisplayNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct DisplayNameSerializer;
impl DisplayNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ETagDeserializer;
impl ETagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ETagSerializer;
impl ETagSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct EmailAddressDeserializer;
impl EmailAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct EmailAddressSerializer;
impl EmailAddressSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct EnableRequestProgressSerializer;
impl EnableRequestProgressSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct EncodingTypeDeserializer;
impl EncodingTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct EncodingTypeSerializer;
impl EncodingTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes the server-side encryption that will be applied to the restore results.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Encryption {
    /// <p>The server-side encryption algorithm used when storing job results in Amazon S3 (e.g., AES256, aws:kms).</p>
    pub encryption_type: String,
    /// <p>If the encryption type is aws:kms, this optional value can be used to specify the encryption context for the restore results.</p>
    pub kms_context: Option<String>,
    /// <p>If the encryption type is aws:kms, this optional value specifies the AWS KMS key ID to use for encryption of job results.</p>
    pub kms_key_id: Option<String>,
}

pub struct EncryptionSerializer;
impl EncryptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Encryption,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("EncryptionType"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.encryption_type
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.kms_context {
            writer.write(xml::writer::XmlEvent::start_element("KMSContext"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.kms_key_id {
            writer.write(xml::writer::XmlEvent::start_element("KMSKeyId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for information regarding encryption based configuration for replicas.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EncryptionConfiguration {
    /// <p>The id of the KMS key used to encrypt the replica object.</p>
    pub replica_kms_key_id: Option<String>,
}

struct EncryptionConfigurationDeserializer;
impl EncryptionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EncryptionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EncryptionConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ReplicaKmsKeyID" => {
                        obj.replica_kms_key_id = Some(try!(
                            ReplicaKmsKeyIDDeserializer::deserialize("ReplicaKmsKeyID", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct EncryptionConfigurationSerializer;
impl EncryptionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &EncryptionConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.replica_kms_key_id {
            writer.write(xml::writer::XmlEvent::start_element("ReplicaKmsKeyID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct EndEvent {}

struct EndEventDeserializer;
impl EndEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EndEvent, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = EndEvent::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3Error {
    pub code: Option<String>,
    pub key: Option<String>,
    pub message: Option<String>,
    pub version_id: Option<String>,
}

struct S3ErrorDeserializer;
impl S3ErrorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3Error, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = S3Error::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Code" => {
                        obj.code = Some(try!(CodeDeserializer::deserialize("Code", stack)));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "Message" => {
                        obj.message =
                            Some(try!(MessageDeserializer::deserialize("Message", stack)));
                    }
                    "VersionId" => {
                        obj.version_id = Some(try!(ObjectVersionIdDeserializer::deserialize(
                            "VersionId",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ErrorDocument {
    /// <p>The object key name to use when a 4XX class error occurs.</p>
    pub key: String,
}

struct ErrorDocumentDeserializer;
impl ErrorDocumentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ErrorDocument, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ErrorDocument::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Key" => {
                        obj.key = try!(ObjectKeyDeserializer::deserialize("Key", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ErrorDocumentSerializer;
impl ErrorDocumentSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ErrorDocument,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ErrorsDeserializer;
impl ErrorsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<S3Error>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(S3ErrorDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
struct EventDeserializer;
impl EventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct EventSerializer;
impl EventSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(EventDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct EventListSerializer;
impl EventListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            EventSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct ExpirationStatusDeserializer;
impl ExpirationStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ExpirationStatusSerializer;
impl ExpirationStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ExpiredObjectDeleteMarkerDeserializer;
impl ExpiredObjectDeleteMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ExpiredObjectDeleteMarkerSerializer;
impl ExpiredObjectDeleteMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ExposeHeaderDeserializer;
impl ExposeHeaderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ExposeHeaderSerializer;
impl ExposeHeaderSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ExposeHeadersDeserializer;
impl ExposeHeadersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ExposeHeaderDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct ExposeHeadersSerializer;
impl ExposeHeadersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            ExposeHeaderSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

pub struct ExpressionSerializer;
impl ExpressionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ExpressionTypeSerializer;
impl ExpressionTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct FetchOwnerSerializer;
impl FetchOwnerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct FieldDelimiterSerializer;
impl FieldDelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct FileHeaderInfoSerializer;
impl FileHeaderInfoSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for key value pair that defines the criteria for the filter rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FilterRule {
    /// <p>Object key name prefix or suffix identifying one or more objects to which the filtering rule applies. Maximum prefix length can be up to 1,024 characters. Overlapping prefixes and suffixes are not supported. For more information, go to <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event Notifications</a> in the Amazon Simple Storage Service Developer Guide.</p>
    pub name: Option<String>,
    pub value: Option<String>,
}

struct FilterRuleDeserializer;
impl FilterRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FilterRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = FilterRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Name" => {
                        obj.name =
                            Some(try!(FilterRuleNameDeserializer::deserialize("Name", stack)));
                    }
                    "Value" => {
                        obj.value = Some(try!(FilterRuleValueDeserializer::deserialize(
                            "Value", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct FilterRuleSerializer;
impl FilterRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &FilterRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.name {
            writer.write(xml::writer::XmlEvent::start_element("Name"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.value {
            writer.write(xml::writer::XmlEvent::start_element("Value"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct FilterRuleListDeserializer;
impl FilterRuleListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<FilterRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(FilterRuleDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct FilterRuleListSerializer;
impl FilterRuleListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<FilterRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            FilterRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct FilterRuleNameDeserializer;
impl FilterRuleNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct FilterRuleNameSerializer;
impl FilterRuleNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct FilterRuleValueDeserializer;
impl FilterRuleValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct FilterRuleValueSerializer;
impl FilterRuleValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAccelerateConfigurationOutput {
    /// <p>The accelerate configuration of the bucket.</p>
    pub status: Option<String>,
}

struct GetBucketAccelerateConfigurationOutputDeserializer;
impl GetBucketAccelerateConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAccelerateConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketAccelerateConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Status" => {
                        obj.status = Some(try!(BucketAccelerateStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAccelerateConfigurationRequest {
    /// <p>Name of the bucket for which the accelerate configuration is retrieved.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAclOutput {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    pub owner: Option<Owner>,
}

struct GetBucketAclOutputDeserializer;
impl GetBucketAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketAclOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AccessControlList" => {
                        obj.grants = Some(try!(GrantsDeserializer::deserialize(
                            "AccessControlList",
                            stack
                        )));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAclRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAnalyticsConfigurationOutput {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

struct GetBucketAnalyticsConfigurationOutputDeserializer;
impl GetBucketAnalyticsConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAnalyticsConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketAnalyticsConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AnalyticsConfiguration" => {
                        obj.analytics_configuration =
                            Some(try!(AnalyticsConfigurationDeserializer::deserialize(
                                "AnalyticsConfiguration",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAnalyticsConfigurationRequest {
    /// <p>The name of the bucket from which an analytics configuration is retrieved.</p>
    pub bucket: String,
    /// <p>The identifier used to represent an analytics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketCorsOutput {
    pub cors_rules: Option<Vec<CORSRule>>,
}

struct GetBucketCorsOutputDeserializer;
impl GetBucketCorsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketCorsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketCorsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CORSRule" => {
                        obj.cors_rules =
                            Some(try!(CORSRulesDeserializer::deserialize("CORSRule", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketCorsRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketEncryptionOutput {
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

struct GetBucketEncryptionOutputDeserializer;
impl GetBucketEncryptionOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketEncryptionOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketEncryptionOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ServerSideEncryptionConfiguration" => {
                        obj.server_side_encryption_configuration = Some(try!(
                            ServerSideEncryptionConfigurationDeserializer::deserialize(
                                "ServerSideEncryptionConfiguration",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketEncryptionRequest {
    /// <p>The name of the bucket from which the server-side encryption configuration is retrieved.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketInventoryConfigurationOutput {
    /// <p>Specifies the inventory configuration.</p>
    pub inventory_configuration: Option<InventoryConfiguration>,
}

struct GetBucketInventoryConfigurationOutputDeserializer;
impl GetBucketInventoryConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketInventoryConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketInventoryConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "InventoryConfiguration" => {
                        obj.inventory_configuration =
                            Some(try!(InventoryConfigurationDeserializer::deserialize(
                                "InventoryConfiguration",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketInventoryConfigurationRequest {
    /// <p>The name of the bucket containing the inventory configuration to retrieve.</p>
    pub bucket: String,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleConfigurationOutput {
    pub rules: Option<Vec<LifecycleRule>>,
}

struct GetBucketLifecycleConfigurationOutputDeserializer;
impl GetBucketLifecycleConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketLifecycleConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Rule" => {
                        obj.rules =
                            Some(try!(LifecycleRulesDeserializer::deserialize("Rule", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleConfigurationRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleOutput {
    pub rules: Option<Vec<Rule>>,
}

struct GetBucketLifecycleOutputDeserializer;
impl GetBucketLifecycleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketLifecycleOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Rule" => {
                        obj.rules = Some(try!(RulesDeserializer::deserialize("Rule", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLocationOutput {
    pub location_constraint: Option<String>,
}

struct GetBucketLocationOutputDeserializer;
impl GetBucketLocationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLocationOutput, XmlParseError> {
        let mut obj = GetBucketLocationOutput::default();
        obj.location_constraint = Some(try!(BucketLocationConstraintDeserializer::deserialize(
            "LocationConstraint",
            stack
        )));
        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLocationRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: Option<LoggingEnabled>,
}

struct GetBucketLoggingOutputDeserializer;
impl GetBucketLoggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLoggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketLoggingOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "LoggingEnabled" => {
                        obj.logging_enabled = Some(try!(LoggingEnabledDeserializer::deserialize(
                            "LoggingEnabled",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLoggingRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketMetricsConfigurationOutput {
    /// <p>Specifies the metrics configuration.</p>
    pub metrics_configuration: Option<MetricsConfiguration>,
}

struct GetBucketMetricsConfigurationOutputDeserializer;
impl GetBucketMetricsConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketMetricsConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketMetricsConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MetricsConfiguration" => {
                        obj.metrics_configuration =
                            Some(try!(MetricsConfigurationDeserializer::deserialize(
                                "MetricsConfiguration",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketMetricsConfigurationRequest {
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    pub bucket: String,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketNotificationConfigurationRequest {
    /// <p>Name of the bucket to get the notification configuration for.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyOutput {
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: Option<ReplicationConfiguration>,
}

struct GetBucketReplicationOutputDeserializer;
impl GetBucketReplicationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketReplicationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketReplicationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ReplicationConfiguration" => {
                        obj.replication_configuration =
                            Some(try!(ReplicationConfigurationDeserializer::deserialize(
                                "ReplicationConfiguration",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketReplicationRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketRequestPaymentOutput {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: Option<String>,
}

struct GetBucketRequestPaymentOutputDeserializer;
impl GetBucketRequestPaymentOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketRequestPaymentOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketRequestPaymentOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Payer" => {
                        obj.payer = Some(try!(PayerDeserializer::deserialize("Payer", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketRequestPaymentRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketTaggingOutput {
    pub tag_set: Vec<Tag>,
}

struct GetBucketTaggingOutputDeserializer;
impl GetBucketTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketTaggingOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "TagSet" => {
                        obj.tag_set = try!(TagSetDeserializer::deserialize("TagSet", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketTaggingRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketVersioningOutput {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<String>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<String>,
}

struct GetBucketVersioningOutputDeserializer;
impl GetBucketVersioningOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketVersioningOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketVersioningOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MfaDelete" => {
                        obj.mfa_delete = Some(try!(MFADeleteStatusDeserializer::deserialize(
                            "MfaDelete",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(BucketVersioningStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketVersioningRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketWebsiteOutput {
    pub error_document: Option<ErrorDocument>,
    pub index_document: Option<IndexDocument>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub routing_rules: Option<Vec<RoutingRule>>,
}

struct GetBucketWebsiteOutputDeserializer;
impl GetBucketWebsiteOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketWebsiteOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketWebsiteOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ErrorDocument" => {
                            obj.error_document = Some(try!(
                                ErrorDocumentDeserializer::deserialize("ErrorDocument", stack)
                            ));
                        }
                        "IndexDocument" => {
                            obj.index_document = Some(try!(
                                IndexDocumentDeserializer::deserialize("IndexDocument", stack)
                            ));
                        }
                        "RedirectAllRequestsTo" => {
                            obj.redirect_all_requests_to =
                                Some(try!(RedirectAllRequestsToDeserializer::deserialize(
                                    "RedirectAllRequestsTo",
                                    stack
                                )));
                        }
                        "RoutingRules" => {
                            obj.routing_rules = Some(try!(RoutingRulesDeserializer::deserialize(
                                "RoutingRules",
                                stack
                            )));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketWebsiteRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectAclOutput {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    pub owner: Option<Owner>,
    pub request_charged: Option<String>,
}

struct GetObjectAclOutputDeserializer;
impl GetObjectAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetObjectAclOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AccessControlList" => {
                        obj.grants = Some(try!(GrantsDeserializer::deserialize(
                            "AccessControlList",
                            stack
                        )));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectAclRequest {
    pub bucket: String,
    pub key: String,
    pub request_payer: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug)]
pub struct GetObjectOutput {
    pub accept_ranges: Option<String>,
    /// <p>Object data.</p>
    pub body: Option<StreamingBody>,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>Size of the body in bytes.</p>
    pub content_length: Option<i64>,
    /// <p>The portion of the object returned in the response.</p>
    pub content_range: Option<String>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If false, this response header does not appear in the response.</p>
    pub delete_marker: Option<bool>,
    /// <p>An ETag is an opaque identifier assigned by a web server to a specific version of a resource found at a URL</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured (see PUT Bucket lifecycle), the response includes this header. It includes the expiry-date and rule-id key value pairs providing object expiration information. The value of the rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Last modified date of the object</p>
    pub last_modified: Option<String>,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>This is set to the number of metadata entries not returned in x-amz-meta headers. This can happen if you create metadata using an API like SOAP that supports more flexible metadata than the REST API. For example, using SOAP, you can create metadata whose values are not legal HTTP headers.</p>
    pub missing_meta: Option<i64>,
    /// <p>The count of parts this object has.</p>
    pub parts_count: Option<i64>,
    pub replication_status: Option<String>,
    pub request_charged: Option<String>,
    /// <p>Provides information about object restoration operation and expiration time of the restored object copy.</p>
    pub restore: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    pub storage_class: Option<String>,
    /// <p>The number of tags, if any, on the object.</p>
    pub tag_count: Option<i64>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectRequest {
    pub bucket: String,
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified, otherwise return a 412 (precondition failed).</p>
    pub if_match: Option<String>,
    /// <p>Return the object only if it has been modified since the specified time, otherwise return a 304 (not modified).</p>
    pub if_modified_since: Option<String>,
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified, otherwise return a 304 (not modified).</p>
    pub if_none_match: Option<String>,
    /// <p>Return the object only if it has not been modified since the specified time, otherwise return a 412 (precondition failed).</p>
    pub if_unmodified_since: Option<String>,
    pub key: String,
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    pub part_number: Option<i64>,
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, go to http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35.</p>
    pub range: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Sets the Cache-Control header of the response.</p>
    pub response_cache_control: Option<String>,
    /// <p>Sets the Content-Disposition header of the response</p>
    pub response_content_disposition: Option<String>,
    /// <p>Sets the Content-Encoding header of the response.</p>
    pub response_content_encoding: Option<String>,
    /// <p>Sets the Content-Language header of the response.</p>
    pub response_content_language: Option<String>,
    /// <p>Sets the Content-Type header of the response.</p>
    pub response_content_type: Option<String>,
    /// <p>Sets the Expires header of the response.</p>
    pub response_expires: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectTaggingOutput {
    pub tag_set: Vec<Tag>,
    pub version_id: Option<String>,
}

struct GetObjectTaggingOutputDeserializer;
impl GetObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetObjectTaggingOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "TagSet" => {
                        obj.tag_set = try!(TagSetDeserializer::deserialize("TagSet", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectTaggingRequest {
    pub bucket: String,
    pub key: String,
    pub version_id: Option<String>,
}

#[derive(Default, Debug)]
pub struct GetObjectTorrentOutput {
    pub body: Option<StreamingBody>,
    pub request_charged: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectTorrentRequest {
    pub bucket: String,
    pub key: String,
    pub request_payer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GlacierJobParameters {
    /// <p>Glacier retrieval tier at which the restore will be processed.</p>
    pub tier: String,
}

pub struct GlacierJobParametersSerializer;
impl GlacierJobParametersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &GlacierJobParameters,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Tier"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.tier
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Grant {
    pub grantee: Option<Grantee>,
    /// <p>Specifies the permission given to the grantee.</p>
    pub permission: Option<String>,
}

struct GrantDeserializer;
impl GrantDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Grant, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Grant::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Grantee" => {
                        obj.grantee =
                            Some(try!(GranteeDeserializer::deserialize("Grantee", stack)));
                    }
                    "Permission" => {
                        obj.permission = Some(try!(PermissionDeserializer::deserialize(
                            "Permission",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct GrantSerializer;
impl GrantSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Grant,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.grantee {
            &GranteeSerializer::serialize(&mut writer, "Grantee", value)?;
        }
        if let Some(ref value) = obj.permission {
            writer.write(xml::writer::XmlEvent::start_element("Permission"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Grantee {
    /// <p>Screen name of the grantee.</p>
    pub display_name: Option<String>,
    /// <p>Email address of the grantee.</p>
    pub email_address: Option<String>,
    /// <p>The canonical user ID of the grantee.</p>
    pub id: Option<String>,
    /// <p>Type of grantee</p>
    pub type_: String,
    /// <p>URI of the grantee group.</p>
    pub uri: Option<String>,
}

struct GranteeDeserializer;
impl GranteeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Grantee, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Grantee::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DisplayName" => {
                        obj.display_name = Some(try!(DisplayNameDeserializer::deserialize(
                            "DisplayName",
                            stack
                        )));
                    }
                    "EmailAddress" => {
                        obj.email_address = Some(try!(EmailAddressDeserializer::deserialize(
                            "EmailAddress",
                            stack
                        )));
                    }
                    "ID" => {
                        obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                    }
                    "xsi:type" => {
                        obj.type_ = try!(TypeDeserializer::deserialize("xsi:type", stack));
                    }
                    "URI" => {
                        obj.uri = Some(try!(URIDeserializer::deserialize("URI", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct GranteeSerializer;
impl GranteeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Grantee,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.display_name {
            writer.write(xml::writer::XmlEvent::start_element("DisplayName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.email_address {
            writer.write(xml::writer::XmlEvent::start_element("EmailAddress"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("xsi:type"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.type_
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.uri {
            writer.write(xml::writer::XmlEvent::start_element("URI"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct GrantsDeserializer;
impl GrantsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Grant>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Grant" {
                        obj.push(try!(GrantDeserializer::deserialize("Grant", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

pub struct GrantsSerializer;
impl GrantsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Grant>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            GrantSerializer::serialize(writer, "Grant", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HeadBucketRequest {
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HeadObjectOutput {
    pub accept_ranges: Option<String>,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>Size of the body in bytes.</p>
    pub content_length: Option<i64>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If false, this response header does not appear in the response.</p>
    pub delete_marker: Option<bool>,
    /// <p>An ETag is an opaque identifier assigned by a web server to a specific version of a resource found at a URL</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured (see PUT Bucket lifecycle), the response includes this header. It includes the expiry-date and rule-id key value pairs providing object expiration information. The value of the rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Last modified date of the object</p>
    pub last_modified: Option<String>,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>This is set to the number of metadata entries not returned in x-amz-meta headers. This can happen if you create metadata using an API like SOAP that supports more flexible metadata than the REST API. For example, using SOAP, you can create metadata whose values are not legal HTTP headers.</p>
    pub missing_meta: Option<i64>,
    /// <p>The count of parts this object has.</p>
    pub parts_count: Option<i64>,
    pub replication_status: Option<String>,
    pub request_charged: Option<String>,
    /// <p>Provides information about object restoration operation and expiration time of the restored object copy.</p>
    pub restore: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    pub storage_class: Option<String>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

struct HeadObjectOutputDeserializer;
impl HeadObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HeadObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = HeadObjectOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HeadObjectRequest {
    pub bucket: String,
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified, otherwise return a 412 (precondition failed).</p>
    pub if_match: Option<String>,
    /// <p>Return the object only if it has been modified since the specified time, otherwise return a 304 (not modified).</p>
    pub if_modified_since: Option<String>,
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified, otherwise return a 304 (not modified).</p>
    pub if_none_match: Option<String>,
    /// <p>Return the object only if it has not been modified since the specified time, otherwise return a 412 (precondition failed).</p>
    pub if_unmodified_since: Option<String>,
    pub key: String,
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' HEAD request for the part specified. Useful querying about the size of the part and the number of parts in this object.</p>
    pub part_number: Option<i64>,
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, go to http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35.</p>
    pub range: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

struct HostNameDeserializer;
impl HostNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct HostNameSerializer;
impl HostNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct HttpErrorCodeReturnedEqualsDeserializer;
impl HttpErrorCodeReturnedEqualsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct HttpErrorCodeReturnedEqualsSerializer;
impl HttpErrorCodeReturnedEqualsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct HttpRedirectCodeDeserializer;
impl HttpRedirectCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct HttpRedirectCodeSerializer;
impl HttpRedirectCodeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct IDDeserializer;
impl IDDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct IDSerializer;
impl IDSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct IndexDocument {
    /// <p>A suffix that is appended to a request that is for a directory on the website endpoint (e.g. if the suffix is index.html and you make a request to samplebucket/images/ the data that is returned will be for the object with the key name images/index.html) The suffix must not be empty and must not include a slash character.</p>
    pub suffix: String,
}

struct IndexDocumentDeserializer;
impl IndexDocumentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexDocument, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IndexDocument::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Suffix" => {
                        obj.suffix = try!(SuffixDeserializer::deserialize("Suffix", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct IndexDocumentSerializer;
impl IndexDocumentSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &IndexDocument,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Suffix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.suffix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InitiatedDeserializer;
impl InitiatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Initiator {
    /// <p>Name of the Principal.</p>
    pub display_name: Option<String>,
    /// <p>If the principal is an AWS account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p>
    pub id: Option<String>,
}

struct InitiatorDeserializer;
impl InitiatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Initiator, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Initiator::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DisplayName" => {
                        obj.display_name = Some(try!(DisplayNameDeserializer::deserialize(
                            "DisplayName",
                            stack
                        )));
                    }
                    "ID" => {
                        obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes the serialization format of the object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InputSerialization {
    /// <p>Describes the serialization of a CSV-encoded object.</p>
    pub csv: Option<CSVInput>,
    /// <p>Specifies object's compression format. Valid values: NONE, GZIP, BZIP2. Default Value: NONE.</p>
    pub compression_type: Option<String>,
    /// <p>Specifies JSON as object's input serialization format.</p>
    pub json: Option<JSONInput>,
}

pub struct InputSerializationSerializer;
impl InputSerializationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InputSerialization,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.csv {
            &CSVInputSerializer::serialize(&mut writer, "CSV", value)?;
        }
        if let Some(ref value) = obj.compression_type {
            writer.write(xml::writer::XmlEvent::start_element("CompressionType"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.json {
            &JSONInputSerializer::serialize(&mut writer, "JSON", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryConfiguration {
    /// <p>Contains information about where to publish the inventory results.</p>
    pub destination: InventoryDestination,
    /// <p>Specifies an inventory filter. The inventory only includes objects that meet the filter's criteria.</p>
    pub filter: Option<InventoryFilter>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
    /// <p>Specifies which object version(s) to included in the inventory results.</p>
    pub included_object_versions: String,
    /// <p>Specifies whether the inventory is enabled or disabled.</p>
    pub is_enabled: bool,
    /// <p>Contains the optional fields that are included in the inventory results.</p>
    pub optional_fields: Option<Vec<String>>,
    /// <p>Specifies the schedule for generating inventory results.</p>
    pub schedule: InventorySchedule,
}

struct InventoryConfigurationDeserializer;
impl InventoryConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Destination" => {
                        obj.destination = try!(InventoryDestinationDeserializer::deserialize(
                            "Destination",
                            stack
                        ));
                    }
                    "Filter" => {
                        obj.filter = Some(try!(InventoryFilterDeserializer::deserialize(
                            "Filter", stack
                        )));
                    }
                    "Id" => {
                        obj.id = try!(InventoryIdDeserializer::deserialize("Id", stack));
                    }
                    "IncludedObjectVersions" => {
                        obj.included_object_versions =
                            try!(InventoryIncludedObjectVersionsDeserializer::deserialize(
                                "IncludedObjectVersions",
                                stack
                            ));
                    }
                    "IsEnabled" => {
                        obj.is_enabled =
                            try!(IsEnabledDeserializer::deserialize("IsEnabled", stack));
                    }
                    "OptionalFields" => {
                        obj.optional_fields =
                            Some(try!(InventoryOptionalFieldsDeserializer::deserialize(
                                "OptionalFields",
                                stack
                            )));
                    }
                    "Schedule" => {
                        obj.schedule = try!(InventoryScheduleDeserializer::deserialize(
                            "Schedule", stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryConfigurationSerializer;
impl InventoryConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        InventoryDestinationSerializer::serialize(&mut writer, "Destination", &obj.destination)?;
        if let Some(ref value) = obj.filter {
            &InventoryFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element(
            "IncludedObjectVersions",
        ))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.included_object_versions
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("IsEnabled"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.is_enabled
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.optional_fields {
            &InventoryOptionalFieldsSerializer::serialize(&mut writer, "OptionalFields", value)?;
        }
        InventoryScheduleSerializer::serialize(&mut writer, "Schedule", &obj.schedule)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryConfigurationListDeserializer;
impl InventoryConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InventoryConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(InventoryConfigurationDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryDestination {
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional) where inventory results are published.</p>
    pub s3_bucket_destination: InventoryS3BucketDestination,
}

struct InventoryDestinationDeserializer;
impl InventoryDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "S3BucketDestination" => {
                        obj.s3_bucket_destination =
                            try!(InventoryS3BucketDestinationDeserializer::deserialize(
                                "S3BucketDestination",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryDestinationSerializer;
impl InventoryDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryDestination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        InventoryS3BucketDestinationSerializer::serialize(
            &mut writer,
            "S3BucketDestination",
            &obj.s3_bucket_destination,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Contains the type of server-side encryption used to encrypt the inventory results.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryEncryption {
    /// <p>Specifies the use of SSE-KMS to encrypt delievered Inventory reports.</p>
    pub ssekms: Option<SSEKMS>,
    /// <p>Specifies the use of SSE-S3 to encrypt delievered Inventory reports.</p>
    pub sses3: Option<SSES3>,
}

struct InventoryEncryptionDeserializer;
impl InventoryEncryptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryEncryption, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryEncryption::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "SSE-KMS" => {
                        obj.ssekms = Some(try!(SSEKMSDeserializer::deserialize("SSE-KMS", stack)));
                    }
                    "SSE-S3" => {
                        obj.sses3 = Some(try!(SSES3Deserializer::deserialize("SSE-S3", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryEncryptionSerializer;
impl InventoryEncryptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryEncryption,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.ssekms {
            &SSEKMSSerializer::serialize(&mut writer, "SSE-KMS", value)?;
        }
        if let Some(ref value) = obj.sses3 {
            &SSES3Serializer::serialize(&mut writer, "SSE-S3", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryFilter {
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub prefix: String,
}

struct InventoryFilterDeserializer;
impl InventoryFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Prefix" => {
                        obj.prefix = try!(PrefixDeserializer::deserialize("Prefix", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryFilterSerializer;
impl InventoryFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryFormatDeserializer;
impl InventoryFormatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryFormatSerializer;
impl InventoryFormatSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryFrequencyDeserializer;
impl InventoryFrequencyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryFrequencySerializer;
impl InventoryFrequencySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryIdDeserializer;
impl InventoryIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryIdSerializer;
impl InventoryIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryIncludedObjectVersionsDeserializer;
impl InventoryIncludedObjectVersionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryIncludedObjectVersionsSerializer;
impl InventoryIncludedObjectVersionsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryOptionalFieldDeserializer;
impl InventoryOptionalFieldDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryOptionalFieldSerializer;
impl InventoryOptionalFieldSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryOptionalFieldsDeserializer;
impl InventoryOptionalFieldsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Field" {
                        obj.push(try!(InventoryOptionalFieldDeserializer::deserialize(
                            "Field", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

pub struct InventoryOptionalFieldsSerializer;
impl InventoryOptionalFieldsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            InventoryOptionalFieldSerializer::serialize(writer, "Field", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryS3BucketDestination {
    /// <p>The ID of the account that owns the destination bucket.</p>
    pub account_id: Option<String>,
    /// <p>The Amazon resource name (ARN) of the bucket where inventory results will be published.</p>
    pub bucket: String,
    /// <p>Contains the type of server-side encryption used to encrypt the inventory results.</p>
    pub encryption: Option<InventoryEncryption>,
    /// <p>Specifies the output format of the inventory results.</p>
    pub format: String,
    /// <p>The prefix that is prepended to all inventory results.</p>
    pub prefix: Option<String>,
}

struct InventoryS3BucketDestinationDeserializer;
impl InventoryS3BucketDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryS3BucketDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryS3BucketDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AccountId" => {
                        obj.account_id =
                            Some(try!(AccountIdDeserializer::deserialize("AccountId", stack)));
                    }
                    "Bucket" => {
                        obj.bucket = try!(BucketNameDeserializer::deserialize("Bucket", stack));
                    }
                    "Encryption" => {
                        obj.encryption = Some(try!(InventoryEncryptionDeserializer::deserialize(
                            "Encryption",
                            stack
                        )));
                    }
                    "Format" => {
                        obj.format =
                            try!(InventoryFormatDeserializer::deserialize("Format", stack));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryS3BucketDestinationSerializer;
impl InventoryS3BucketDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryS3BucketDestination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.account_id {
            writer.write(xml::writer::XmlEvent::start_element("AccountId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.encryption {
            &InventoryEncryptionSerializer::serialize(&mut writer, "Encryption", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Format"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.format
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventorySchedule {
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub frequency: String,
}

struct InventoryScheduleDeserializer;
impl InventoryScheduleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventorySchedule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventorySchedule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Frequency" => {
                        obj.frequency = try!(InventoryFrequencyDeserializer::deserialize(
                            "Frequency",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct InventoryScheduleSerializer;
impl InventoryScheduleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventorySchedule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Frequency"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.frequency
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct IsEnabledDeserializer;
impl IsEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct IsEnabledSerializer;
impl IsEnabledSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct IsLatestDeserializer;
impl IsLatestDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IsTruncatedDeserializer;
impl IsTruncatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct JSONInput {
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub type_: Option<String>,
}

pub struct JSONInputSerializer;
impl JSONInputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &JSONInput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.type_ {
            writer.write(xml::writer::XmlEvent::start_element("Type"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct JSONOutput {
    /// <p>The value used to separate individual records in the output.</p>
    pub record_delimiter: Option<String>,
}

pub struct JSONOutputSerializer;
impl JSONOutputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &JSONOutput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.record_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("RecordDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct JSONTypeSerializer;
impl JSONTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct KMSContextSerializer;
impl KMSContextSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct KeyCountDeserializer;
impl KeyCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct KeyMarkerDeserializer;
impl KeyMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct KeyMarkerSerializer;
impl KeyMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct KeyPrefixEqualsDeserializer;
impl KeyPrefixEqualsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct KeyPrefixEqualsSerializer;
impl KeyPrefixEqualsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LambdaFunctionArnDeserializer;
impl LambdaFunctionArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct LambdaFunctionArnSerializer;
impl LambdaFunctionArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for specifying the AWS Lambda notification configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LambdaFunctionConfiguration {
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>Lambda cloud function ARN that Amazon S3 can invoke when it detects events of the specified type.</p>
    pub lambda_function_arn: String,
}

struct LambdaFunctionConfigurationDeserializer;
impl LambdaFunctionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaFunctionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LambdaFunctionConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Event" => {
                        obj.events = try!(EventListDeserializer::deserialize("Event", stack));
                    }
                    "Filter" => {
                        obj.filter = Some(try!(
                            NotificationConfigurationFilterDeserializer::deserialize(
                                "Filter", stack
                            )
                        ));
                    }
                    "Id" => {
                        obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id", stack)));
                    }
                    "CloudFunction" => {
                        obj.lambda_function_arn = try!(LambdaFunctionArnDeserializer::deserialize(
                            "CloudFunction",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct LambdaFunctionConfigurationSerializer;
impl LambdaFunctionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LambdaFunctionConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("CloudFunction"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.lambda_function_arn
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LambdaFunctionConfigurationListDeserializer;
impl LambdaFunctionConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LambdaFunctionConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(LambdaFunctionConfigurationDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct LambdaFunctionConfigurationListSerializer;
impl LambdaFunctionConfigurationListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<LambdaFunctionConfiguration>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            LambdaFunctionConfigurationSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct LastModifiedDeserializer;
impl LastModifiedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleConfiguration {
    pub rules: Vec<Rule>,
}

pub struct LifecycleConfigurationSerializer;
impl LifecycleConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        RulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleExpiration {
    /// <p>Indicates at what date the object is to be moved or deleted. Should be in GMT ISO 8601 Format.</p>
    pub date: Option<String>,
    /// <p>Indicates the lifetime, in days, of the objects that are subject to the rule. The value must be a non-zero positive integer.</p>
    pub days: Option<i64>,
    /// <p>Indicates whether Amazon S3 will remove a delete marker with no noncurrent versions. If set to true, the delete marker will be expired; if set to false the policy takes no action. This cannot be specified with Days or Date in a Lifecycle Expiration Policy.</p>
    pub expired_object_delete_marker: Option<bool>,
}

struct LifecycleExpirationDeserializer;
impl LifecycleExpirationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleExpiration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Date" => {
                        obj.date = Some(try!(DateDeserializer::deserialize("Date", stack)));
                    }
                    "Days" => {
                        obj.days = Some(try!(DaysDeserializer::deserialize("Days", stack)));
                    }
                    "ExpiredObjectDeleteMarker" => {
                        obj.expired_object_delete_marker =
                            Some(try!(ExpiredObjectDeleteMarkerDeserializer::deserialize(
                                "ExpiredObjectDeleteMarker",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct LifecycleExpirationSerializer;
impl LifecycleExpirationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleExpiration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.date {
            writer.write(xml::writer::XmlEvent::start_element("Date"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.expired_object_delete_marker {
            writer.write(xml::writer::XmlEvent::start_element(
                "ExpiredObjectDeleteMarker",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleRule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub expiration: Option<LifecycleExpiration>,
    pub filter: Option<LifecycleRuleFilter>,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: Option<String>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub noncurrent_version_transitions: Option<Vec<NoncurrentVersionTransition>>,
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    pub status: String,
    pub transitions: Option<Vec<Transition>>,
}

struct LifecycleRuleDeserializer;
impl LifecycleRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AbortIncompleteMultipartUpload" => {
                        obj.abort_incomplete_multipart_upload = Some(try!(
                            AbortIncompleteMultipartUploadDeserializer::deserialize(
                                "AbortIncompleteMultipartUpload",
                                stack
                            )
                        ));
                    }
                    "Expiration" => {
                        obj.expiration = Some(try!(LifecycleExpirationDeserializer::deserialize(
                            "Expiration",
                            stack
                        )));
                    }
                    "Filter" => {
                        obj.filter = Some(try!(LifecycleRuleFilterDeserializer::deserialize(
                            "Filter", stack
                        )));
                    }
                    "ID" => {
                        obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                    }
                    "NoncurrentVersionExpiration" => {
                        obj.noncurrent_version_expiration =
                            Some(try!(NoncurrentVersionExpirationDeserializer::deserialize(
                                "NoncurrentVersionExpiration",
                                stack
                            )));
                    }
                    "NoncurrentVersionTransition" => {
                        obj.noncurrent_version_transitions = Some(try!(
                            NoncurrentVersionTransitionListDeserializer::deserialize(
                                "NoncurrentVersionTransition",
                                stack
                            )
                        ));
                    }
                    "Status" => {
                        obj.status =
                            try!(ExpirationStatusDeserializer::deserialize("Status", stack));
                    }
                    "Transition" => {
                        obj.transitions = Some(try!(TransitionListDeserializer::deserialize(
                            "Transition",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct LifecycleRuleSerializer;
impl LifecycleRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.abort_incomplete_multipart_upload {
            &AbortIncompleteMultipartUploadSerializer::serialize(
                &mut writer,
                "AbortIncompleteMultipartUpload",
                value,
            )?;
        }
        if let Some(ref value) = obj.expiration {
            &LifecycleExpirationSerializer::serialize(&mut writer, "Expiration", value)?;
        }
        if let Some(ref value) = obj.filter {
            &LifecycleRuleFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.noncurrent_version_expiration {
            &NoncurrentVersionExpirationSerializer::serialize(
                &mut writer,
                "NoncurrentVersionExpiration",
                value,
            )?;
        }
        if let Some(ref value) = obj.noncurrent_version_transitions {
            &NoncurrentVersionTransitionListSerializer::serialize(
                &mut writer,
                "NoncurrentVersionTransition",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Status"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.status
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.transitions {
            &TransitionListSerializer::serialize(&mut writer, "Transition", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>This is used in a Lifecycle Rule Filter to apply a logical AND to two or more predicates. The Lifecycle Rule will apply to any object matching all of the predicates configured inside the And operator.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleRuleAndOperator {
    pub prefix: Option<String>,
    /// <p>All of these tags must exist in the object's tag set in order for the rule to apply.</p>
    pub tags: Option<Vec<Tag>>,
}

struct LifecycleRuleAndOperatorDeserializer;
impl LifecycleRuleAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRuleAndOperator, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleRuleAndOperator::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "Tag" => {
                        obj.tags = Some(try!(TagSetDeserializer::deserialize("Tag", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct LifecycleRuleAndOperatorSerializer;
impl LifecycleRuleAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleRuleAndOperator,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The Filter is used to identify objects that a Lifecycle Rule applies to. A Filter must have exactly one of Prefix, Tag, or And specified.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleRuleFilter {
    pub and: Option<LifecycleRuleAndOperator>,
    /// <p>Prefix identifying one or more objects to which the rule applies.</p>
    pub prefix: Option<String>,
    /// <p>This tag must exist in the object's tag set in order for the rule to apply.</p>
    pub tag: Option<Tag>,
}

struct LifecycleRuleFilterDeserializer;
impl LifecycleRuleFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRuleFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleRuleFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "And" => {
                        obj.and = Some(try!(LifecycleRuleAndOperatorDeserializer::deserialize(
                            "And", stack
                        )));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "Tag" => {
                        obj.tag = Some(try!(TagDeserializer::deserialize("Tag", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct LifecycleRuleFilterSerializer;
impl LifecycleRuleFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleRuleFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &LifecycleRuleAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tag {
            &TagSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LifecycleRulesDeserializer;
impl LifecycleRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LifecycleRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(LifecycleRuleDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct LifecycleRulesSerializer;
impl LifecycleRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<LifecycleRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            LifecycleRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketAnalyticsConfigurationsOutput {
    /// <p>The list of analytics configurations for a bucket.</p>
    pub analytics_configuration_list: Option<Vec<AnalyticsConfiguration>>,
    /// <p>The ContinuationToken that represents where this request began.</p>
    pub continuation_token: Option<String>,
    /// <p>Indicates whether the returned list of analytics configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken will be provided for a subsequent request.</p>
    pub is_truncated: Option<bool>,
    /// <p>NextContinuationToken is sent when isTruncated is true, which indicates that there are more analytics configurations to list. The next request must include this NextContinuationToken. The token is obfuscated and is not a usable value.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketAnalyticsConfigurationsOutputDeserializer;
impl ListBucketAnalyticsConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketAnalyticsConfigurationsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListBucketAnalyticsConfigurationsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AnalyticsConfiguration" => {
                        obj.analytics_configuration_list =
                            Some(try!(AnalyticsConfigurationListDeserializer::deserialize(
                                "AnalyticsConfiguration",
                                stack
                            )));
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(try!(TokenDeserializer::deserialize(
                            "ContinuationToken",
                            stack
                        )));
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(try!(
                            NextTokenDeserializer::deserialize("NextContinuationToken", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketAnalyticsConfigurationsRequest {
    /// <p>The name of the bucket from which analytics configurations are retrieved.</p>
    pub bucket: String,
    /// <p>The ContinuationToken that represents a placeholder from where this request should begin.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketInventoryConfigurationsOutput {
    /// <p>If sent in the request, the marker that is used as a starting point for this inventory configuration list response.</p>
    pub continuation_token: Option<String>,
    /// <p>The list of inventory configurations for a bucket.</p>
    pub inventory_configuration_list: Option<Vec<InventoryConfiguration>>,
    /// <p>Indicates whether the returned list of inventory configurations is truncated in this response. A value of true indicates that the list is truncated.</p>
    pub is_truncated: Option<bool>,
    /// <p>The marker used to continue this inventory configuration listing. Use the NextContinuationToken from this response to continue the listing in a subsequent request. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketInventoryConfigurationsOutputDeserializer;
impl ListBucketInventoryConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketInventoryConfigurationsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListBucketInventoryConfigurationsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ContinuationToken" => {
                        obj.continuation_token = Some(try!(TokenDeserializer::deserialize(
                            "ContinuationToken",
                            stack
                        )));
                    }
                    "InventoryConfiguration" => {
                        obj.inventory_configuration_list =
                            Some(try!(InventoryConfigurationListDeserializer::deserialize(
                                "InventoryConfiguration",
                                stack
                            )));
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(try!(
                            NextTokenDeserializer::deserialize("NextContinuationToken", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketInventoryConfigurationsRequest {
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub bucket: String,
    /// <p>The marker used to continue an inventory configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketMetricsConfigurationsOutput {
    /// <p>The marker that is used as a starting point for this metrics configuration list response. This value is present if it was sent in the request.</p>
    pub continuation_token: Option<String>,
    /// <p>Indicates whether the returned list of metrics configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken will be provided for a subsequent request.</p>
    pub is_truncated: Option<bool>,
    /// <p>The list of metrics configurations for a bucket.</p>
    pub metrics_configuration_list: Option<Vec<MetricsConfiguration>>,
    /// <p>The marker used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketMetricsConfigurationsOutputDeserializer;
impl ListBucketMetricsConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketMetricsConfigurationsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListBucketMetricsConfigurationsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ContinuationToken" => {
                        obj.continuation_token = Some(try!(TokenDeserializer::deserialize(
                            "ContinuationToken",
                            stack
                        )));
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "MetricsConfiguration" => {
                        obj.metrics_configuration_list =
                            Some(try!(MetricsConfigurationListDeserializer::deserialize(
                                "MetricsConfiguration",
                                stack
                            )));
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(try!(
                            NextTokenDeserializer::deserialize("NextContinuationToken", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketMetricsConfigurationsRequest {
    /// <p>The name of the bucket containing the metrics configurations to retrieve.</p>
    pub bucket: String,
    /// <p>The marker that is used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketsOutput {
    pub buckets: Option<Vec<Bucket>>,
    pub owner: Option<Owner>,
}

struct ListBucketsOutputDeserializer;
impl ListBucketsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListBucketsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Buckets" => {
                        obj.buckets =
                            Some(try!(BucketsDeserializer::deserialize("Buckets", stack)));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListMultipartUploadsOutput {
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: Option<String>,
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>Indicates whether the returned list of multipart uploads is truncated. A value of true indicates that the list was truncated. The list can be truncated if the number of multipart uploads exceeds the limit allowed or specified by max uploads.</p>
    pub is_truncated: Option<bool>,
    /// <p>The key at or after which the listing began.</p>
    pub key_marker: Option<String>,
    /// <p>Maximum number of multipart uploads that could have been included in the response.</p>
    pub max_uploads: Option<i64>,
    /// <p>When a list is truncated, this element specifies the value that should be used for the key-marker request parameter in a subsequent request.</p>
    pub next_key_marker: Option<String>,
    /// <p>When a list is truncated, this element specifies the value that should be used for the upload-id-marker request parameter in a subsequent request.</p>
    pub next_upload_id_marker: Option<String>,
    /// <p>When a prefix is provided in the request, this field contains the specified prefix. The result contains only keys starting with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Upload ID after which listing began.</p>
    pub upload_id_marker: Option<String>,
    pub uploads: Option<Vec<MultipartUpload>>,
}

struct ListMultipartUploadsOutputDeserializer;
impl ListMultipartUploadsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListMultipartUploadsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListMultipartUploadsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Bucket" => {
                        obj.bucket =
                            Some(try!(BucketNameDeserializer::deserialize("Bucket", stack)));
                    }
                    "CommonPrefixes" => {
                        obj.common_prefixes = Some(try!(
                            CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)
                        ));
                    }
                    "Delimiter" => {
                        obj.delimiter =
                            Some(try!(DelimiterDeserializer::deserialize("Delimiter", stack)));
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(try!(EncodingTypeDeserializer::deserialize(
                            "EncodingType",
                            stack
                        )));
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "KeyMarker" => {
                        obj.key_marker =
                            Some(try!(KeyMarkerDeserializer::deserialize("KeyMarker", stack)));
                    }
                    "MaxUploads" => {
                        obj.max_uploads = Some(try!(MaxUploadsDeserializer::deserialize(
                            "MaxUploads",
                            stack
                        )));
                    }
                    "NextKeyMarker" => {
                        obj.next_key_marker = Some(try!(NextKeyMarkerDeserializer::deserialize(
                            "NextKeyMarker",
                            stack
                        )));
                    }
                    "NextUploadIdMarker" => {
                        obj.next_upload_id_marker =
                            Some(try!(NextUploadIdMarkerDeserializer::deserialize(
                                "NextUploadIdMarker",
                                stack
                            )));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "UploadIdMarker" => {
                        obj.upload_id_marker = Some(try!(UploadIdMarkerDeserializer::deserialize(
                            "UploadIdMarker",
                            stack
                        )));
                    }
                    "Upload" => {
                        obj.uploads = Some(try!(MultipartUploadListDeserializer::deserialize(
                            "Upload", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListMultipartUploadsRequest {
    pub bucket: String,
    /// <p>Character you use to group keys.</p>
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    /// <p>Together with upload-id-marker, this parameter specifies the multipart upload after which listing should begin.</p>
    pub key_marker: Option<String>,
    /// <p>Sets the maximum number of multipart uploads, from 1 to 1,000, to return in the response body. 1,000 is the maximum number of uploads that can be returned in a response.</p>
    pub max_uploads: Option<i64>,
    /// <p>Lists in-progress uploads only for those keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Together with key-marker, specifies the multipart upload after which listing should begin. If key-marker is not specified, the upload-id-marker parameter is ignored.</p>
    pub upload_id_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectVersionsOutput {
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    pub delete_markers: Option<Vec<DeleteMarkerEntry>>,
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria. If your results were truncated, you can make a follow-up paginated request using the NextKeyMarker and NextVersionIdMarker response parameters as a starting place in another request to return the rest of the results.</p>
    pub is_truncated: Option<bool>,
    /// <p>Marks the last Key returned in a truncated response.</p>
    pub key_marker: Option<String>,
    pub max_keys: Option<i64>,
    pub name: Option<String>,
    /// <p>Use this value for the key marker request parameter in a subsequent request.</p>
    pub next_key_marker: Option<String>,
    /// <p>Use this value for the next version id marker parameter in a subsequent request.</p>
    pub next_version_id_marker: Option<String>,
    pub prefix: Option<String>,
    pub version_id_marker: Option<String>,
    pub versions: Option<Vec<ObjectVersion>>,
}

struct ListObjectVersionsOutputDeserializer;
impl ListObjectVersionsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectVersionsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListObjectVersionsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CommonPrefixes" => {
                        obj.common_prefixes = Some(try!(
                            CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)
                        ));
                    }
                    "DeleteMarker" => {
                        obj.delete_markers = Some(try!(DeleteMarkersDeserializer::deserialize(
                            "DeleteMarker",
                            stack
                        )));
                    }
                    "Delimiter" => {
                        obj.delimiter =
                            Some(try!(DelimiterDeserializer::deserialize("Delimiter", stack)));
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(try!(EncodingTypeDeserializer::deserialize(
                            "EncodingType",
                            stack
                        )));
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "KeyMarker" => {
                        obj.key_marker =
                            Some(try!(KeyMarkerDeserializer::deserialize("KeyMarker", stack)));
                    }
                    "MaxKeys" => {
                        obj.max_keys =
                            Some(try!(MaxKeysDeserializer::deserialize("MaxKeys", stack)));
                    }
                    "Name" => {
                        obj.name = Some(try!(BucketNameDeserializer::deserialize("Name", stack)));
                    }
                    "NextKeyMarker" => {
                        obj.next_key_marker = Some(try!(NextKeyMarkerDeserializer::deserialize(
                            "NextKeyMarker",
                            stack
                        )));
                    }
                    "NextVersionIdMarker" => {
                        obj.next_version_id_marker =
                            Some(try!(NextVersionIdMarkerDeserializer::deserialize(
                                "NextVersionIdMarker",
                                stack
                            )));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "VersionIdMarker" => {
                        obj.version_id_marker = Some(try!(
                            VersionIdMarkerDeserializer::deserialize("VersionIdMarker", stack)
                        ));
                    }
                    "Version" => {
                        obj.versions = Some(try!(ObjectVersionListDeserializer::deserialize(
                            "Version", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectVersionsRequest {
    pub bucket: String,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    /// <p>Specifies the key to start with when listing objects in a bucket.</p>
    pub key_marker: Option<String>,
    /// <p>Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more.</p>
    pub max_keys: Option<i64>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Specifies the object version you want to start listing from.</p>
    pub version_id_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectsOutput {
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    pub contents: Option<Vec<Object>>,
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria.</p>
    pub is_truncated: Option<bool>,
    pub marker: Option<String>,
    pub max_keys: Option<i64>,
    pub name: Option<String>,
    /// <p>When response is truncated (the IsTruncated element value in the response is true), you can use the key name in this field as marker in the subsequent request to get next set of objects. Amazon S3 lists objects in alphabetical order Note: This element is returned only if you have delimiter request parameter specified. If response does not include the NextMaker and it is truncated, you can use the value of the last Key in the response as the marker in the subsequent request to get the next set of object keys.</p>
    pub next_marker: Option<String>,
    pub prefix: Option<String>,
}

struct ListObjectsOutputDeserializer;
impl ListObjectsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListObjectsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CommonPrefixes" => {
                        obj.common_prefixes = Some(try!(
                            CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)
                        ));
                    }
                    "Contents" => {
                        obj.contents =
                            Some(try!(ObjectListDeserializer::deserialize("Contents", stack)));
                    }
                    "Delimiter" => {
                        obj.delimiter =
                            Some(try!(DelimiterDeserializer::deserialize("Delimiter", stack)));
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(try!(EncodingTypeDeserializer::deserialize(
                            "EncodingType",
                            stack
                        )));
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(MarkerDeserializer::deserialize("Marker", stack)));
                    }
                    "MaxKeys" => {
                        obj.max_keys =
                            Some(try!(MaxKeysDeserializer::deserialize("MaxKeys", stack)));
                    }
                    "Name" => {
                        obj.name = Some(try!(BucketNameDeserializer::deserialize("Name", stack)));
                    }
                    "NextMarker" => {
                        obj.next_marker = Some(try!(NextMarkerDeserializer::deserialize(
                            "NextMarker",
                            stack
                        )));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectsRequest {
    pub bucket: String,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    /// <p>Specifies the key to start with when listing objects in a bucket.</p>
    pub marker: Option<String>,
    /// <p>Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more.</p>
    pub max_keys: Option<i64>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub request_payer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectsV2Output {
    /// <p>CommonPrefixes contains all (if there are any) keys between Prefix and the next occurrence of the string specified by delimiter</p>
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    /// <p>Metadata about each object returned.</p>
    pub contents: Option<Vec<Object>>,
    /// <p>ContinuationToken indicates Amazon S3 that the list is being continued on this bucket with a token. ContinuationToken is obfuscated and is not a real key</p>
    pub continuation_token: Option<String>,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria.</p>
    pub is_truncated: Option<bool>,
    /// <p>KeyCount is the number of keys returned with this request. KeyCount will always be less than equals to MaxKeys field. Say you ask for 50 keys, your result will include less than equals 50 keys </p>
    pub key_count: Option<i64>,
    /// <p>Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more.</p>
    pub max_keys: Option<i64>,
    /// <p>Name of the bucket to list.</p>
    pub name: Option<String>,
    /// <p>NextContinuationToken is sent when isTruncated is true which means there are more keys in the bucket that can be listed. The next list requests to Amazon S3 can be continued with this NextContinuationToken. NextContinuationToken is obfuscated and is not a real key</p>
    pub next_continuation_token: Option<String>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>StartAfter is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. StartAfter can be any key in the bucket</p>
    pub start_after: Option<String>,
}

struct ListObjectsV2OutputDeserializer;
impl ListObjectsV2OutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsV2Output, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListObjectsV2Output::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CommonPrefixes" => {
                        obj.common_prefixes = Some(try!(
                            CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)
                        ));
                    }
                    "Contents" => {
                        obj.contents =
                            Some(try!(ObjectListDeserializer::deserialize("Contents", stack)));
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(try!(TokenDeserializer::deserialize(
                            "ContinuationToken",
                            stack
                        )));
                    }
                    "Delimiter" => {
                        obj.delimiter =
                            Some(try!(DelimiterDeserializer::deserialize("Delimiter", stack)));
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(try!(EncodingTypeDeserializer::deserialize(
                            "EncodingType",
                            stack
                        )));
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "KeyCount" => {
                        obj.key_count =
                            Some(try!(KeyCountDeserializer::deserialize("KeyCount", stack)));
                    }
                    "MaxKeys" => {
                        obj.max_keys =
                            Some(try!(MaxKeysDeserializer::deserialize("MaxKeys", stack)));
                    }
                    "Name" => {
                        obj.name = Some(try!(BucketNameDeserializer::deserialize("Name", stack)));
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(try!(
                            NextTokenDeserializer::deserialize("NextContinuationToken", stack)
                        ));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "StartAfter" => {
                        obj.start_after = Some(try!(StartAfterDeserializer::deserialize(
                            "StartAfter",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectsV2Request {
    /// <p>Name of the bucket to list.</p>
    pub bucket: String,
    /// <p>ContinuationToken indicates Amazon S3 that the list is being continued on this bucket with a token. ContinuationToken is obfuscated and is not a real key</p>
    pub continuation_token: Option<String>,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>The owner field is not present in listV2 by default, if you want to return owner field with each key in the result then set the fetch owner field to true</p>
    pub fetch_owner: Option<bool>,
    /// <p>Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more.</p>
    pub max_keys: Option<i64>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request in V2 style. Bucket owners need not specify this parameter in their requests.</p>
    pub request_payer: Option<String>,
    /// <p>StartAfter is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. StartAfter can be any key in the bucket</p>
    pub start_after: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPartsOutput {
    /// <p>Date when multipart upload will become eligible for abort operation by lifecycle.</p>
    pub abort_date: Option<String>,
    /// <p>Id of the lifecycle rule that makes a multipart upload eligible for abort operation.</p>
    pub abort_rule_id: Option<String>,
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: Option<String>,
    /// <p>Identifies who initiated the multipart upload.</p>
    pub initiator: Option<Initiator>,
    /// <p>Indicates whether the returned list of parts is truncated.</p>
    pub is_truncated: Option<bool>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: Option<String>,
    /// <p>Maximum number of parts that were allowed in the response.</p>
    pub max_parts: Option<i64>,
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as the value to use for the part-number-marker request parameter in a subsequent request.</p>
    pub next_part_number_marker: Option<i64>,
    pub owner: Option<Owner>,
    /// <p>Part number after which listing begins.</p>
    pub part_number_marker: Option<i64>,
    pub parts: Option<Vec<Part>>,
    pub request_charged: Option<String>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub upload_id: Option<String>,
}

struct ListPartsOutputDeserializer;
impl ListPartsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPartsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListPartsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Bucket" => {
                        obj.bucket =
                            Some(try!(BucketNameDeserializer::deserialize("Bucket", stack)));
                    }
                    "Initiator" => {
                        obj.initiator =
                            Some(try!(InitiatorDeserializer::deserialize("Initiator", stack)));
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "MaxParts" => {
                        obj.max_parts =
                            Some(try!(MaxPartsDeserializer::deserialize("MaxParts", stack)));
                    }
                    "NextPartNumberMarker" => {
                        obj.next_part_number_marker =
                            Some(try!(NextPartNumberMarkerDeserializer::deserialize(
                                "NextPartNumberMarker",
                                stack
                            )));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                    }
                    "PartNumberMarker" => {
                        obj.part_number_marker = Some(try!(
                            PartNumberMarkerDeserializer::deserialize("PartNumberMarker", stack)
                        ));
                    }
                    "Part" => {
                        obj.parts = Some(try!(PartsDeserializer::deserialize("Part", stack)));
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(try!(StorageClassDeserializer::deserialize(
                            "StorageClass",
                            stack
                        )));
                    }
                    "UploadId" => {
                        obj.upload_id = Some(try!(MultipartUploadIdDeserializer::deserialize(
                            "UploadId", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPartsRequest {
    pub bucket: String,
    pub key: String,
    /// <p>Sets the maximum number of parts to return.</p>
    pub max_parts: Option<i64>,
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub part_number_marker: Option<i64>,
    pub request_payer: Option<String>,
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub upload_id: String,
}

struct LocationDeserializer;
impl LocationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct LocationPrefixSerializer;
impl LocationPrefixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for logging information. Presence of this element indicates that logging is enabled. Parameters TargetBucket and TargetPrefix are required in this case.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoggingEnabled {
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case you should choose a different TargetPrefix for each source bucket so that the delivered log files can be distinguished by key.</p>
    pub target_bucket: String,
    pub target_grants: Option<Vec<TargetGrant>>,
    /// <p>This element lets you specify a prefix for the keys that the log files will be stored under.</p>
    pub target_prefix: String,
}

struct LoggingEnabledDeserializer;
impl LoggingEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoggingEnabled, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoggingEnabled::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "TargetBucket" => {
                        obj.target_bucket =
                            try!(TargetBucketDeserializer::deserialize("TargetBucket", stack));
                    }
                    "TargetGrants" => {
                        obj.target_grants = Some(try!(TargetGrantsDeserializer::deserialize(
                            "TargetGrants",
                            stack
                        )));
                    }
                    "TargetPrefix" => {
                        obj.target_prefix =
                            try!(TargetPrefixDeserializer::deserialize("TargetPrefix", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct LoggingEnabledSerializer;
impl LoggingEnabledSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LoggingEnabled,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("TargetBucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.target_bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.target_grants {
            &TargetGrantsSerializer::serialize(&mut writer, "TargetGrants", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("TargetPrefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.target_prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct MFADeleteSerializer;
impl MFADeleteSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MFADeleteStatusDeserializer;
impl MFADeleteStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MarkerDeserializer;
impl MarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MarkerSerializer;
impl MarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MaxAgeSecondsDeserializer;
impl MaxAgeSecondsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MaxAgeSecondsSerializer;
impl MaxAgeSecondsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MaxKeysDeserializer;
impl MaxKeysDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MaxKeysSerializer;
impl MaxKeysSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MaxPartsDeserializer;
impl MaxPartsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MaxPartsSerializer;
impl MaxPartsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MaxUploadsDeserializer;
impl MaxUploadsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MaxUploadsSerializer;
impl MaxUploadsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MessageDeserializer;
impl MessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A metadata key-value pair to store with an object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetadataEntry {
    pub name: Option<String>,
    pub value: Option<String>,
}

pub struct MetadataEntrySerializer;
impl MetadataEntrySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MetadataEntry,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.name {
            writer.write(xml::writer::XmlEvent::start_element("Name"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.value {
            writer.write(xml::writer::XmlEvent::start_element("Value"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct MetadataKeySerializer;
impl MetadataKeySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct MetadataValueSerializer;
impl MetadataValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricsAndOperator {
    /// <p>The prefix used when evaluating an AND predicate.</p>
    pub prefix: Option<String>,
    /// <p>The list of tags used when evaluating an AND predicate.</p>
    pub tags: Option<Vec<Tag>>,
}

struct MetricsAndOperatorDeserializer;
impl MetricsAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsAndOperator, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricsAndOperator::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "Tag" => {
                        obj.tags = Some(try!(TagSetDeserializer::deserialize("Tag", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MetricsAndOperatorSerializer;
impl MetricsAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MetricsAndOperator,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricsConfiguration {
    /// <p>Specifies a metrics configuration filter. The metrics configuration will only include objects that meet the filter's criteria. A filter must be a prefix, a tag, or a conjunction (MetricsAndOperator).</p>
    pub filter: Option<MetricsFilter>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
}

struct MetricsConfigurationDeserializer;
impl MetricsConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricsConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Filter" => {
                        obj.filter = Some(try!(MetricsFilterDeserializer::deserialize(
                            "Filter", stack
                        )));
                    }
                    "Id" => {
                        obj.id = try!(MetricsIdDeserializer::deserialize("Id", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MetricsConfigurationSerializer;
impl MetricsConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MetricsConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter {
            &MetricsFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MetricsConfigurationListDeserializer;
impl MetricsConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricsConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(MetricsConfigurationDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricsFilter {
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates, and an object must match all of the predicates in order for the filter to apply.</p>
    pub and: Option<MetricsAndOperator>,
    /// <p>The prefix used when evaluating a metrics filter.</p>
    pub prefix: Option<String>,
    /// <p>The tag used when evaluating a metrics filter.</p>
    pub tag: Option<Tag>,
}

struct MetricsFilterDeserializer;
impl MetricsFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricsFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "And" => {
                        obj.and = Some(try!(MetricsAndOperatorDeserializer::deserialize(
                            "And", stack
                        )));
                    }
                    "Prefix" => {
                        obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix", stack)));
                    }
                    "Tag" => {
                        obj.tag = Some(try!(TagDeserializer::deserialize("Tag", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MetricsFilterSerializer;
impl MetricsFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MetricsFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &MetricsAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tag {
            &TagSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MetricsIdDeserializer;
impl MetricsIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MetricsIdSerializer;
impl MetricsIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MultipartUpload {
    /// <p>Date and time at which the multipart upload was initiated.</p>
    pub initiated: Option<String>,
    /// <p>Identifies who initiated the multipart upload.</p>
    pub initiator: Option<Initiator>,
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub key: Option<String>,
    pub owner: Option<Owner>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub upload_id: Option<String>,
}

struct MultipartUploadDeserializer;
impl MultipartUploadDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MultipartUpload, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MultipartUpload::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Initiated" => {
                        obj.initiated =
                            Some(try!(InitiatedDeserializer::deserialize("Initiated", stack)));
                    }
                    "Initiator" => {
                        obj.initiator =
                            Some(try!(InitiatorDeserializer::deserialize("Initiator", stack)));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(try!(StorageClassDeserializer::deserialize(
                            "StorageClass",
                            stack
                        )));
                    }
                    "UploadId" => {
                        obj.upload_id = Some(try!(MultipartUploadIdDeserializer::deserialize(
                            "UploadId", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MultipartUploadIdDeserializer;
impl MultipartUploadIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct MultipartUploadIdSerializer;
impl MultipartUploadIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MultipartUploadListDeserializer;
impl MultipartUploadListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MultipartUpload>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(MultipartUploadDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
struct NextKeyMarkerDeserializer;
impl NextKeyMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct NextMarkerDeserializer;
impl NextMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct NextPartNumberMarkerDeserializer;
impl NextPartNumberMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct NextTokenDeserializer;
impl NextTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct NextUploadIdMarkerDeserializer;
impl NextUploadIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct NextVersionIdMarkerDeserializer;
impl NextVersionIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NoncurrentVersionExpiration {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-access-control.html">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the Amazon Simple Storage Service Developer Guide.</p>
    pub noncurrent_days: Option<i64>,
}

struct NoncurrentVersionExpirationDeserializer;
impl NoncurrentVersionExpirationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoncurrentVersionExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NoncurrentVersionExpiration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "NoncurrentDays" => {
                        obj.noncurrent_days =
                            Some(try!(DaysDeserializer::deserialize("NoncurrentDays", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct NoncurrentVersionExpirationSerializer;
impl NoncurrentVersionExpirationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NoncurrentVersionExpiration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.noncurrent_days {
            writer.write(xml::writer::XmlEvent::start_element("NoncurrentDays"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for the transition rule that describes when noncurrent objects transition to the STANDARD_IA, ONEZONE_IA or GLACIER storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to the STANDARD_IA, ONEZONE_IA or GLACIER storage class at a specific period in the object's lifetime.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NoncurrentVersionTransition {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-access-control.html">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the Amazon Simple Storage Service Developer Guide.</p>
    pub noncurrent_days: Option<i64>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
}

struct NoncurrentVersionTransitionDeserializer;
impl NoncurrentVersionTransitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoncurrentVersionTransition, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NoncurrentVersionTransition::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "NoncurrentDays" => {
                        obj.noncurrent_days =
                            Some(try!(DaysDeserializer::deserialize("NoncurrentDays", stack)));
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(try!(
                            TransitionStorageClassDeserializer::deserialize("StorageClass", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct NoncurrentVersionTransitionSerializer;
impl NoncurrentVersionTransitionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NoncurrentVersionTransition,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.noncurrent_days {
            writer.write(xml::writer::XmlEvent::start_element("NoncurrentDays"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.storage_class {
            writer.write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct NoncurrentVersionTransitionListDeserializer;
impl NoncurrentVersionTransitionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NoncurrentVersionTransition>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(NoncurrentVersionTransitionDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct NoncurrentVersionTransitionListSerializer;
impl NoncurrentVersionTransitionListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<NoncurrentVersionTransition>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            NoncurrentVersionTransitionSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p>Container for specifying the notification configuration of the bucket. If this element is empty, notifications are turned off on the bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotificationConfiguration {
    pub lambda_function_configurations: Option<Vec<LambdaFunctionConfiguration>>,
    pub queue_configurations: Option<Vec<QueueConfiguration>>,
    pub topic_configurations: Option<Vec<TopicConfiguration>>,
}

struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NotificationConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFunctionConfiguration" => {
                        obj.lambda_function_configurations = Some(try!(
                            LambdaFunctionConfigurationListDeserializer::deserialize(
                                "CloudFunctionConfiguration",
                                stack
                            )
                        ));
                    }
                    "QueueConfiguration" => {
                        obj.queue_configurations =
                            Some(try!(QueueConfigurationListDeserializer::deserialize(
                                "QueueConfiguration",
                                stack
                            )));
                    }
                    "TopicConfiguration" => {
                        obj.topic_configurations =
                            Some(try!(TopicConfigurationListDeserializer::deserialize(
                                "TopicConfiguration",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct NotificationConfigurationSerializer;
impl NotificationConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NotificationConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.lambda_function_configurations {
            &LambdaFunctionConfigurationListSerializer::serialize(
                &mut writer,
                "CloudFunctionConfiguration",
                value,
            )?;
        }
        if let Some(ref value) = obj.queue_configurations {
            &QueueConfigurationListSerializer::serialize(&mut writer, "QueueConfiguration", value)?;
        }
        if let Some(ref value) = obj.topic_configurations {
            &TopicConfigurationListSerializer::serialize(&mut writer, "TopicConfiguration", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotificationConfigurationDeprecated {
    pub cloud_function_configuration: Option<CloudFunctionConfiguration>,
    pub queue_configuration: Option<QueueConfigurationDeprecated>,
    pub topic_configuration: Option<TopicConfigurationDeprecated>,
}

struct NotificationConfigurationDeprecatedDeserializer;
impl NotificationConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NotificationConfigurationDeprecated::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFunctionConfiguration" => {
                        obj.cloud_function_configuration =
                            Some(try!(CloudFunctionConfigurationDeserializer::deserialize(
                                "CloudFunctionConfiguration",
                                stack
                            )));
                    }
                    "QueueConfiguration" => {
                        obj.queue_configuration =
                            Some(try!(QueueConfigurationDeprecatedDeserializer::deserialize(
                                "QueueConfiguration",
                                stack
                            )));
                    }
                    "TopicConfiguration" => {
                        obj.topic_configuration =
                            Some(try!(TopicConfigurationDeprecatedDeserializer::deserialize(
                                "TopicConfiguration",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct NotificationConfigurationDeprecatedSerializer;
impl NotificationConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NotificationConfigurationDeprecated,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.cloud_function_configuration {
            &CloudFunctionConfigurationSerializer::serialize(
                &mut writer,
                "CloudFunctionConfiguration",
                value,
            )?;
        }
        if let Some(ref value) = obj.queue_configuration {
            &QueueConfigurationDeprecatedSerializer::serialize(
                &mut writer,
                "QueueConfiguration",
                value,
            )?;
        }
        if let Some(ref value) = obj.topic_configuration {
            &TopicConfigurationDeprecatedSerializer::serialize(
                &mut writer,
                "TopicConfiguration",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for object key name filtering rules. For information about key name filtering, go to <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event Notifications</a> in the Amazon Simple Storage Service Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotificationConfigurationFilter {
    pub key: Option<S3KeyFilter>,
}

struct NotificationConfigurationFilterDeserializer;
impl NotificationConfigurationFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfigurationFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NotificationConfigurationFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "S3Key" => {
                        obj.key = Some(try!(S3KeyFilterDeserializer::deserialize("S3Key", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct NotificationConfigurationFilterSerializer;
impl NotificationConfigurationFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NotificationConfigurationFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.key {
            &S3KeyFilterSerializer::serialize(&mut writer, "S3Key", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct NotificationIdDeserializer;
impl NotificationIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct NotificationIdSerializer;
impl NotificationIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Object {
    pub e_tag: Option<String>,
    pub key: Option<String>,
    pub last_modified: Option<String>,
    pub owner: Option<Owner>,
    pub size: Option<i64>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
}

struct ObjectDeserializer;
impl ObjectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Object, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Object::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ETag" => {
                        obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "LastModified" => {
                        obj.last_modified = Some(try!(LastModifiedDeserializer::deserialize(
                            "LastModified",
                            stack
                        )));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                    }
                    "Size" => {
                        obj.size = Some(try!(SizeDeserializer::deserialize("Size", stack)));
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(try!(
                            ObjectStorageClassDeserializer::deserialize("StorageClass", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ObjectCannedACLSerializer;
impl ObjectCannedACLSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectIdentifier {
    /// <p>Key name of the object to delete.</p>
    pub key: String,
    /// <p>VersionId for the specific version of the object to delete.</p>
    pub version_id: Option<String>,
}

pub struct ObjectIdentifierSerializer;
impl ObjectIdentifierSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ObjectIdentifier,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.version_id {
            writer.write(xml::writer::XmlEvent::start_element("VersionId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ObjectIdentifierListSerializer;
impl ObjectIdentifierListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<ObjectIdentifier>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            ObjectIdentifierSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct ObjectKeyDeserializer;
impl ObjectKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ObjectKeySerializer;
impl ObjectKeySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ObjectListDeserializer;
impl ObjectListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Object>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ObjectDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
struct ObjectStorageClassDeserializer;
impl ObjectStorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectVersion {
    pub e_tag: Option<String>,
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub is_latest: Option<bool>,
    /// <p>The object key.</p>
    pub key: Option<String>,
    /// <p>Date and time the object was last modified.</p>
    pub last_modified: Option<String>,
    pub owner: Option<Owner>,
    /// <p>Size in bytes of the object.</p>
    pub size: Option<i64>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
    /// <p>Version ID of an object.</p>
    pub version_id: Option<String>,
}

struct ObjectVersionDeserializer;
impl ObjectVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectVersion, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ObjectVersion::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ETag" => {
                        obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                    }
                    "IsLatest" => {
                        obj.is_latest =
                            Some(try!(IsLatestDeserializer::deserialize("IsLatest", stack)));
                    }
                    "Key" => {
                        obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                    }
                    "LastModified" => {
                        obj.last_modified = Some(try!(LastModifiedDeserializer::deserialize(
                            "LastModified",
                            stack
                        )));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                    }
                    "Size" => {
                        obj.size = Some(try!(SizeDeserializer::deserialize("Size", stack)));
                    }
                    "StorageClass" => {
                        obj.storage_class =
                            Some(try!(ObjectVersionStorageClassDeserializer::deserialize(
                                "StorageClass",
                                stack
                            )));
                    }
                    "VersionId" => {
                        obj.version_id = Some(try!(ObjectVersionIdDeserializer::deserialize(
                            "VersionId",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ObjectVersionIdDeserializer;
impl ObjectVersionIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ObjectVersionIdSerializer;
impl ObjectVersionIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ObjectVersionListDeserializer;
impl ObjectVersionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ObjectVersion>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ObjectVersionDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
struct ObjectVersionStorageClassDeserializer;
impl ObjectVersionStorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes the location where the restore job's output is stored.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OutputLocation {
    /// <p>Describes an S3 location that will receive the results of the restore request.</p>
    pub s3: Option<S3Location>,
}

pub struct OutputLocationSerializer;
impl OutputLocationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OutputLocation,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.s3 {
            &S3LocationSerializer::serialize(&mut writer, "S3", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes how results of the Select job are serialized.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OutputSerialization {
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    pub csv: Option<CSVOutput>,
    /// <p>Specifies JSON as request's output serialization format.</p>
    pub json: Option<JSONOutput>,
}

pub struct OutputSerializationSerializer;
impl OutputSerializationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OutputSerialization,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.csv {
            &CSVOutputSerializer::serialize(&mut writer, "CSV", value)?;
        }
        if let Some(ref value) = obj.json {
            &JSONOutputSerializer::serialize(&mut writer, "JSON", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Owner {
    pub display_name: Option<String>,
    pub id: Option<String>,
}

struct OwnerDeserializer;
impl OwnerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Owner, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Owner::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DisplayName" => {
                        obj.display_name = Some(try!(DisplayNameDeserializer::deserialize(
                            "DisplayName",
                            stack
                        )));
                    }
                    "ID" => {
                        obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct OwnerSerializer;
impl OwnerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Owner,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.display_name {
            writer.write(xml::writer::XmlEvent::start_element("DisplayName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct OwnerOverrideDeserializer;
impl OwnerOverrideDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct OwnerOverrideSerializer;
impl OwnerOverrideSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Part {
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub e_tag: Option<String>,
    /// <p>Date and time at which the part was uploaded.</p>
    pub last_modified: Option<String>,
    /// <p>Part number identifying the part. This is a positive integer between 1 and 10,000.</p>
    pub part_number: Option<i64>,
    /// <p>Size of the uploaded part data.</p>
    pub size: Option<i64>,
}

struct PartDeserializer;
impl PartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Part, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Part::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ETag" => {
                        obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                    }
                    "LastModified" => {
                        obj.last_modified = Some(try!(LastModifiedDeserializer::deserialize(
                            "LastModified",
                            stack
                        )));
                    }
                    "PartNumber" => {
                        obj.part_number = Some(try!(PartNumberDeserializer::deserialize(
                            "PartNumber",
                            stack
                        )));
                    }
                    "Size" => {
                        obj.size = Some(try!(SizeDeserializer::deserialize("Size", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PartNumberDeserializer;
impl PartNumberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct PartNumberSerializer;
impl PartNumberSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PartNumberMarkerDeserializer;
impl PartNumberMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct PartNumberMarkerSerializer;
impl PartNumberMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PartsDeserializer;
impl PartsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Part>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(PartDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
struct PayerDeserializer;
impl PayerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct PayerSerializer;
impl PayerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PermissionDeserializer;
impl PermissionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct PermissionSerializer;
impl PermissionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct PolicySerializer;
impl PolicySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PrefixDeserializer;
impl PrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct PrefixSerializer;
impl PrefixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Progress {
    /// <p>Current number of uncompressed object bytes processed.</p>
    pub bytes_processed: Option<i64>,
    /// <p>Current number of bytes of records payload data returned.</p>
    pub bytes_returned: Option<i64>,
    /// <p>Current number of object bytes scanned.</p>
    pub bytes_scanned: Option<i64>,
}

struct ProgressDeserializer;
impl ProgressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Progress, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Progress::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "BytesProcessed" => {
                            obj.bytes_processed = Some(try!(
                                BytesProcessedDeserializer::deserialize("BytesProcessed", stack)
                            ));
                        }
                        "BytesReturned" => {
                            obj.bytes_returned = Some(try!(
                                BytesReturnedDeserializer::deserialize("BytesReturned", stack)
                            ));
                        }
                        "BytesScanned" => {
                            obj.bytes_scanned = Some(try!(BytesScannedDeserializer::deserialize(
                                "BytesScanned",
                                stack
                            )));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProgressEvent {
    /// <p>The Progress event details.</p>
    pub details: Option<Progress>,
}

struct ProgressEventDeserializer;
impl ProgressEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ProgressEvent, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ProgressEvent::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Details" => {
                        obj.details =
                            Some(try!(ProgressDeserializer::deserialize("Details", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ProtocolDeserializer;
impl ProtocolDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ProtocolSerializer;
impl ProtocolSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAccelerateConfigurationRequest {
    /// <p>Specifies the Accelerate Configuration you want to set for the bucket.</p>
    pub accelerate_configuration: AccelerateConfiguration,
    /// <p>Name of the bucket for which the accelerate configuration is set.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAclRequest {
    /// <p>The canned ACL to apply to the bucket.</p>
    pub acl: Option<String>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub bucket: String,
    pub content_md5: Option<String>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to list the objects in the bucket.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to create, overwrite, and delete any object in the bucket.</p>
    pub grant_write: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    pub grant_write_acp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAnalyticsConfigurationRequest {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: AnalyticsConfiguration,
    /// <p>The name of the bucket to which an analytics configuration is stored.</p>
    pub bucket: String,
    /// <p>The identifier used to represent an analytics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketCorsRequest {
    pub bucket: String,
    pub cors_configuration: CORSConfiguration,
    pub content_md5: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketEncryptionRequest {
    /// <p>The name of the bucket for which the server-side encryption configuration is set.</p>
    pub bucket: String,
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
    pub content_md5: Option<String>,
    pub server_side_encryption_configuration: ServerSideEncryptionConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketInventoryConfigurationRequest {
    /// <p>The name of the bucket where the inventory configuration will be stored.</p>
    pub bucket: String,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
    /// <p>Specifies the inventory configuration.</p>
    pub inventory_configuration: InventoryConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLifecycleConfigurationRequest {
    pub bucket: String,
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLifecycleRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub lifecycle_configuration: Option<LifecycleConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLoggingRequest {
    pub bucket: String,
    pub bucket_logging_status: BucketLoggingStatus,
    pub content_md5: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketMetricsConfigurationRequest {
    /// <p>The name of the bucket for which the metrics configuration is set.</p>
    pub bucket: String,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
    /// <p>Specifies the metrics configuration.</p>
    pub metrics_configuration: MetricsConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketNotificationConfigurationRequest {
    pub bucket: String,
    pub notification_configuration: NotificationConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketNotificationRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub notification_configuration: NotificationConfigurationDeprecated,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketPolicyRequest {
    pub bucket: String,
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p>
    pub confirm_remove_self_bucket_access: Option<bool>,
    pub content_md5: Option<String>,
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketReplicationRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub replication_configuration: ReplicationConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketRequestPaymentRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub request_payment_configuration: RequestPaymentConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketTaggingRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub tagging: Tagging,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketVersioningRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub mfa: Option<String>,
    pub versioning_configuration: VersioningConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketWebsiteRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub website_configuration: WebsiteConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectAclOutput {
    pub request_charged: Option<String>,
}

struct PutObjectAclOutputDeserializer;
impl PutObjectAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = PutObjectAclOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectAclRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub bucket: String,
    pub content_md5: Option<String>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to list the objects in the bucket.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to create, overwrite, and delete any object in the bucket.</p>
    pub grant_write: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    pub grant_write_acp: Option<String>,
    pub key: String,
    pub request_payer: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectOutput {
    /// <p>Entity tag for the uploaded object.</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured, this will contain the expiration date (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
}

struct PutObjectOutputDeserializer;
impl PutObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = PutObjectOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug)]
pub struct PutObjectRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    /// <p>Object data.</p>
    pub body: Option<StreamingBody>,
    /// <p>Name of the bucket to which the PUT operation was initiated.</p>
    pub bucket: String,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically.</p>
    pub content_length: Option<i64>,
    /// <p>The base64-encoded 128-bit MD5 digest of the part data.</p>
    pub content_md5: Option<String>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to read the object data and its metadata.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the object ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable object.</p>
    pub grant_write_acp: Option<String>,
    /// <p>Object key for which the PUT operation was initiated.</p>
    pub key: String,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>The type of storage to use for the object. Defaults to 'STANDARD'.</p>
    pub storage_class: Option<String>,
    /// <p>The tag-set for the object. The tag-set must be encoded as URL Query parameters</p>
    pub tagging: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectTaggingOutput {
    pub version_id: Option<String>,
}

struct PutObjectTaggingOutputDeserializer;
impl PutObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = PutObjectTaggingOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectTaggingRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub key: String,
    pub tagging: Tagging,
    pub version_id: Option<String>,
}

struct QueueArnDeserializer;
impl QueueArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct QueueArnSerializer;
impl QueueArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for specifying an configuration when you want Amazon S3 to publish events to an Amazon Simple Queue Service (Amazon SQS) queue.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueueConfiguration {
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>Amazon SQS queue ARN to which Amazon S3 will publish a message when it detects events of specified type.</p>
    pub queue_arn: String,
}

struct QueueConfigurationDeserializer;
impl QueueConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueueConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = QueueConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Event" => {
                        obj.events = try!(EventListDeserializer::deserialize("Event", stack));
                    }
                    "Filter" => {
                        obj.filter = Some(try!(
                            NotificationConfigurationFilterDeserializer::deserialize(
                                "Filter", stack
                            )
                        ));
                    }
                    "Id" => {
                        obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id", stack)));
                    }
                    "Queue" => {
                        obj.queue_arn = try!(QueueArnDeserializer::deserialize("Queue", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct QueueConfigurationSerializer;
impl QueueConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueueConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Queue"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.queue_arn
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueueConfigurationDeprecated {
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    pub queue: Option<String>,
}

struct QueueConfigurationDeprecatedDeserializer;
impl QueueConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueueConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = QueueConfigurationDeprecated::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Event" => {
                        obj.events = Some(try!(EventListDeserializer::deserialize("Event", stack)));
                    }
                    "Id" => {
                        obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id", stack)));
                    }
                    "Queue" => {
                        obj.queue = Some(try!(QueueArnDeserializer::deserialize("Queue", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct QueueConfigurationDeprecatedSerializer;
impl QueueConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueueConfigurationDeprecated,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.queue {
            writer.write(xml::writer::XmlEvent::start_element("Queue"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct QueueConfigurationListDeserializer;
impl QueueConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<QueueConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(QueueConfigurationDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct QueueConfigurationListSerializer;
impl QueueConfigurationListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<QueueConfiguration>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            QueueConfigurationSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

pub struct QuietSerializer;
impl QuietSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct QuoteCharacterSerializer;
impl QuoteCharacterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct QuoteEscapeCharacterSerializer;
impl QuoteEscapeCharacterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct QuoteFieldsSerializer;
impl QuoteFieldsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct RecordDelimiterSerializer;
impl RecordDelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RecordsEvent {
    /// <p>The byte array of partial, one or more result records.</p>
    pub payload: Option<Vec<u8>>,
}

struct RecordsEventDeserializer;
impl RecordsEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RecordsEvent, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RecordsEvent::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Payload" => {
                        obj.payload = Some(try!(BodyDeserializer::deserialize("Payload", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Redirect {
    /// <p>The host name to use in the redirect request.</p>
    pub host_name: Option<String>,
    /// <p>The HTTP redirect code to use on the response. Not required if one of the siblings is present.</p>
    pub http_redirect_code: Option<String>,
    /// <p>Protocol to use (http, https) when redirecting requests. The default is the protocol that is used in the original request.</p>
    pub protocol: Option<String>,
    /// <p>The object key prefix to use in the redirect request. For example, to redirect requests for all pages with prefix docs/ (objects in the docs/ folder) to documents/, you can set a condition block with KeyPrefixEquals set to docs/ and in the Redirect set ReplaceKeyPrefixWith to /documents. Not required if one of the siblings is present. Can be present only if ReplaceKeyWith is not provided.</p>
    pub replace_key_prefix_with: Option<String>,
    /// <p>The specific object key to use in the redirect request. For example, redirect request to error.html. Not required if one of the sibling is present. Can be present only if ReplaceKeyPrefixWith is not provided.</p>
    pub replace_key_with: Option<String>,
}

struct RedirectDeserializer;
impl RedirectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Redirect, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Redirect::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HostName" => {
                        obj.host_name =
                            Some(try!(HostNameDeserializer::deserialize("HostName", stack)));
                    }
                    "HttpRedirectCode" => {
                        obj.http_redirect_code = Some(try!(
                            HttpRedirectCodeDeserializer::deserialize("HttpRedirectCode", stack)
                        ));
                    }
                    "Protocol" => {
                        obj.protocol =
                            Some(try!(ProtocolDeserializer::deserialize("Protocol", stack)));
                    }
                    "ReplaceKeyPrefixWith" => {
                        obj.replace_key_prefix_with =
                            Some(try!(ReplaceKeyPrefixWithDeserializer::deserialize(
                                "ReplaceKeyPrefixWith",
                                stack
                            )));
                    }
                    "ReplaceKeyWith" => {
                        obj.replace_key_with = Some(try!(ReplaceKeyWithDeserializer::deserialize(
                            "ReplaceKeyWith",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct RedirectSerializer;
impl RedirectSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Redirect,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.host_name {
            writer.write(xml::writer::XmlEvent::start_element("HostName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.http_redirect_code {
            writer.write(xml::writer::XmlEvent::start_element("HttpRedirectCode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.protocol {
            writer.write(xml::writer::XmlEvent::start_element("Protocol"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.replace_key_prefix_with {
            writer.write(xml::writer::XmlEvent::start_element("ReplaceKeyPrefixWith"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.replace_key_with {
            writer.write(xml::writer::XmlEvent::start_element("ReplaceKeyWith"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RedirectAllRequestsTo {
    /// <p>Name of the host where requests will be redirected.</p>
    pub host_name: String,
    /// <p>Protocol to use (http, https) when redirecting requests. The default is the protocol that is used in the original request.</p>
    pub protocol: Option<String>,
}

struct RedirectAllRequestsToDeserializer;
impl RedirectAllRequestsToDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RedirectAllRequestsTo, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RedirectAllRequestsTo::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HostName" => {
                        obj.host_name = try!(HostNameDeserializer::deserialize("HostName", stack));
                    }
                    "Protocol" => {
                        obj.protocol =
                            Some(try!(ProtocolDeserializer::deserialize("Protocol", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct RedirectAllRequestsToSerializer;
impl RedirectAllRequestsToSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RedirectAllRequestsTo,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("HostName"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.host_name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.protocol {
            writer.write(xml::writer::XmlEvent::start_element("Protocol"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplaceKeyPrefixWithDeserializer;
impl ReplaceKeyPrefixWithDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ReplaceKeyPrefixWithSerializer;
impl ReplaceKeyPrefixWithSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplaceKeyWithDeserializer;
impl ReplaceKeyWithDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ReplaceKeyWithSerializer;
impl ReplaceKeyWithSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplicaKmsKeyIDDeserializer;
impl ReplicaKmsKeyIDDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ReplicaKmsKeyIDSerializer;
impl ReplicaKmsKeyIDSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for replication rules. You can add as many as 1,000 rules. Total replication configuration size can be up to 2 MB.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplicationConfiguration {
    /// <p>Amazon Resource Name (ARN) of an IAM role for Amazon S3 to assume when replicating the objects.</p>
    pub role: String,
    /// <p>Container for information about a particular replication rule. Replication configuration must have at least one rule and can contain up to 1,000 rules.</p>
    pub rules: Vec<ReplicationRule>,
}

struct ReplicationConfigurationDeserializer;
impl ReplicationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReplicationConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Role" => {
                        obj.role = try!(RoleDeserializer::deserialize("Role", stack));
                    }
                    "Rule" => {
                        obj.rules = try!(ReplicationRulesDeserializer::deserialize("Rule", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ReplicationConfigurationSerializer;
impl ReplicationConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ReplicationConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Role"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.role
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        ReplicationRulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for information about a particular replication rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplicationRule {
    /// <p>Container for replication destination information.</p>
    pub destination: Destination,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: Option<String>,
    /// <p>Object keyname prefix identifying one or more objects to which the rule applies. Maximum prefix length can be up to 1,024 characters. Overlapping prefixes are not supported.</p>
    pub prefix: String,
    /// <p>Container for filters that define which source objects should be replicated.</p>
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    /// <p>The rule is ignored if status is not Enabled.</p>
    pub status: String,
}

struct ReplicationRuleDeserializer;
impl ReplicationRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReplicationRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Destination" => {
                        obj.destination =
                            try!(DestinationDeserializer::deserialize("Destination", stack));
                    }
                    "ID" => {
                        obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                    }
                    "Prefix" => {
                        obj.prefix = try!(PrefixDeserializer::deserialize("Prefix", stack));
                    }
                    "SourceSelectionCriteria" => {
                        obj.source_selection_criteria =
                            Some(try!(SourceSelectionCriteriaDeserializer::deserialize(
                                "SourceSelectionCriteria",
                                stack
                            )));
                    }
                    "Status" => {
                        obj.status = try!(ReplicationRuleStatusDeserializer::deserialize(
                            "Status", stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ReplicationRuleSerializer;
impl ReplicationRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ReplicationRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        DestinationSerializer::serialize(&mut writer, "Destination", &obj.destination)?;
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.source_selection_criteria {
            &SourceSelectionCriteriaSerializer::serialize(
                &mut writer,
                "SourceSelectionCriteria",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Status"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.status
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplicationRuleStatusDeserializer;
impl ReplicationRuleStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ReplicationRuleStatusSerializer;
impl ReplicationRuleStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplicationRulesDeserializer;
impl ReplicationRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReplicationRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ReplicationRuleDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct ReplicationRulesSerializer;
impl ReplicationRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<ReplicationRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            ReplicationRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RequestPaymentConfiguration {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: String,
}

pub struct RequestPaymentConfigurationSerializer;
impl RequestPaymentConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RequestPaymentConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Payer"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.payer
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RequestProgress {
    /// <p>Specifies whether periodic QueryProgress frames should be sent. Valid values: TRUE, FALSE. Default value: FALSE.</p>
    pub enabled: Option<bool>,
}

pub struct RequestProgressSerializer;
impl RequestProgressSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RequestProgress,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.enabled {
            writer.write(xml::writer::XmlEvent::start_element("Enabled"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseCacheControlSerializer;
impl ResponseCacheControlSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseContentDispositionSerializer;
impl ResponseContentDispositionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseContentEncodingSerializer;
impl ResponseContentEncodingSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseContentLanguageSerializer;
impl ResponseContentLanguageSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseContentTypeSerializer;
impl ResponseContentTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseExpiresSerializer;
impl ResponseExpiresSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreObjectOutput {
    pub request_charged: Option<String>,
    /// <p>Indicates the path in the provided S3 output location where Select results will be restored to.</p>
    pub restore_output_path: Option<String>,
}

struct RestoreObjectOutputDeserializer;
impl RestoreObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = RestoreObjectOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreObjectRequest {
    pub bucket: String,
    pub key: String,
    pub request_payer: Option<String>,
    pub restore_request: Option<RestoreRequest>,
    pub version_id: Option<String>,
}

/// <p>Container for restore job parameters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreRequest {
    /// <p>Lifetime of the active copy in days. Do not use with restores that specify OutputLocation.</p>
    pub days: Option<i64>,
    /// <p>The optional description for the job.</p>
    pub description: Option<String>,
    /// <p>Glacier related parameters pertaining to this job. Do not use with restores that specify OutputLocation.</p>
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    /// <p>Describes the location where the restore job's output is stored.</p>
    pub output_location: Option<OutputLocation>,
    /// <p>Describes the parameters for Select job types.</p>
    pub select_parameters: Option<SelectParameters>,
    /// <p>Glacier retrieval tier at which the restore will be processed.</p>
    pub tier: Option<String>,
    /// <p>Type of restore request.</p>
    pub type_: Option<String>,
}

pub struct RestoreRequestSerializer;
impl RestoreRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RestoreRequest,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.description {
            writer.write(xml::writer::XmlEvent::start_element("Description"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.glacier_job_parameters {
            &GlacierJobParametersSerializer::serialize(&mut writer, "GlacierJobParameters", value)?;
        }
        if let Some(ref value) = obj.output_location {
            &OutputLocationSerializer::serialize(&mut writer, "OutputLocation", value)?;
        }
        if let Some(ref value) = obj.select_parameters {
            &SelectParametersSerializer::serialize(&mut writer, "SelectParameters", value)?;
        }
        if let Some(ref value) = obj.tier {
            writer.write(xml::writer::XmlEvent::start_element("Tier"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.type_ {
            writer.write(xml::writer::XmlEvent::start_element("Type"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct RestoreRequestTypeSerializer;
impl RestoreRequestTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct RoleDeserializer;
impl RoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct RoleSerializer;
impl RoleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RoutingRule {
    /// <p>A container for describing a condition that must be met for the specified redirect to apply. For example, 1. If request is for pages in the /docs folder, redirect to the /documents folder. 2. If request results in HTTP error 4xx, redirect request to another host where you might process the error.</p>
    pub condition: Option<Condition>,
    /// <p>Container for redirect information. You can redirect requests to another host, to another page, or with another protocol. In the event of an error, you can can specify a different error code to return.</p>
    pub redirect: Redirect,
}

struct RoutingRuleDeserializer;
impl RoutingRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RoutingRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RoutingRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Condition" => {
                        obj.condition =
                            Some(try!(ConditionDeserializer::deserialize("Condition", stack)));
                    }
                    "Redirect" => {
                        obj.redirect = try!(RedirectDeserializer::deserialize("Redirect", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct RoutingRuleSerializer;
impl RoutingRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RoutingRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.condition {
            &ConditionSerializer::serialize(&mut writer, "Condition", value)?;
        }
        RedirectSerializer::serialize(&mut writer, "Redirect", &obj.redirect)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct RoutingRulesDeserializer;
impl RoutingRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RoutingRule>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "RoutingRule" {
                        obj.push(try!(RoutingRuleDeserializer::deserialize(
                            "RoutingRule",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

pub struct RoutingRulesSerializer;
impl RoutingRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<RoutingRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            RoutingRuleSerializer::serialize(writer, "RoutingRule", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub expiration: Option<LifecycleExpiration>,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: Option<String>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,
    /// <p>Prefix identifying one or more objects to which the rule applies.</p>
    pub prefix: String,
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    pub status: String,
    pub transition: Option<Transition>,
}

struct RuleDeserializer;
impl RuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Rule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Rule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AbortIncompleteMultipartUpload" => {
                        obj.abort_incomplete_multipart_upload = Some(try!(
                            AbortIncompleteMultipartUploadDeserializer::deserialize(
                                "AbortIncompleteMultipartUpload",
                                stack
                            )
                        ));
                    }
                    "Expiration" => {
                        obj.expiration = Some(try!(LifecycleExpirationDeserializer::deserialize(
                            "Expiration",
                            stack
                        )));
                    }
                    "ID" => {
                        obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                    }
                    "NoncurrentVersionExpiration" => {
                        obj.noncurrent_version_expiration =
                            Some(try!(NoncurrentVersionExpirationDeserializer::deserialize(
                                "NoncurrentVersionExpiration",
                                stack
                            )));
                    }
                    "NoncurrentVersionTransition" => {
                        obj.noncurrent_version_transition =
                            Some(try!(NoncurrentVersionTransitionDeserializer::deserialize(
                                "NoncurrentVersionTransition",
                                stack
                            )));
                    }
                    "Prefix" => {
                        obj.prefix = try!(PrefixDeserializer::deserialize("Prefix", stack));
                    }
                    "Status" => {
                        obj.status =
                            try!(ExpirationStatusDeserializer::deserialize("Status", stack));
                    }
                    "Transition" => {
                        obj.transition = Some(try!(TransitionDeserializer::deserialize(
                            "Transition",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct RuleSerializer;
impl RuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Rule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.abort_incomplete_multipart_upload {
            &AbortIncompleteMultipartUploadSerializer::serialize(
                &mut writer,
                "AbortIncompleteMultipartUpload",
                value,
            )?;
        }
        if let Some(ref value) = obj.expiration {
            &LifecycleExpirationSerializer::serialize(&mut writer, "Expiration", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.noncurrent_version_expiration {
            &NoncurrentVersionExpirationSerializer::serialize(
                &mut writer,
                "NoncurrentVersionExpiration",
                value,
            )?;
        }
        if let Some(ref value) = obj.noncurrent_version_transition {
            &NoncurrentVersionTransitionSerializer::serialize(
                &mut writer,
                "NoncurrentVersionTransition",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Status"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.status
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.transition {
            &TransitionSerializer::serialize(&mut writer, "Transition", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct RulesDeserializer;
impl RulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Rule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(RuleDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct RulesSerializer;
impl RulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Rule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            RuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p>Container for object key name prefix and suffix filtering rules.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3KeyFilter {
    pub filter_rules: Option<Vec<FilterRule>>,
}

struct S3KeyFilterDeserializer;
impl S3KeyFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3KeyFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = S3KeyFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "FilterRule" => {
                        obj.filter_rules = Some(try!(FilterRuleListDeserializer::deserialize(
                            "FilterRule",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct S3KeyFilterSerializer;
impl S3KeyFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &S3KeyFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter_rules {
            &FilterRuleListSerializer::serialize(&mut writer, "FilterRule", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes an S3 location that will receive the results of the restore request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3Location {
    /// <p>A list of grants that control access to the staged results.</p>
    pub access_control_list: Option<Vec<Grant>>,
    /// <p>The name of the bucket where the restore results will be placed.</p>
    pub bucket_name: String,
    /// <p>The canned ACL to apply to the restore results.</p>
    pub canned_acl: Option<String>,
    pub encryption: Option<Encryption>,
    /// <p>The prefix that is prepended to the restore results for this request.</p>
    pub prefix: String,
    /// <p>The class of storage used to store the restore results.</p>
    pub storage_class: Option<String>,
    /// <p>The tag-set that is applied to the restore results.</p>
    pub tagging: Option<Tagging>,
    /// <p>A list of metadata to store with the restore results in S3.</p>
    pub user_metadata: Option<Vec<MetadataEntry>>,
}

pub struct S3LocationSerializer;
impl S3LocationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &S3Location,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.access_control_list {
            &GrantsSerializer::serialize(&mut writer, "AccessControlList", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("BucketName"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket_name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.canned_acl {
            writer.write(xml::writer::XmlEvent::start_element("CannedACL"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.encryption {
            &EncryptionSerializer::serialize(&mut writer, "Encryption", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.storage_class {
            writer.write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tagging {
            &TaggingSerializer::serialize(&mut writer, "Tagging", value)?;
        }
        if let Some(ref value) = obj.user_metadata {
            &UserMetadataSerializer::serialize(&mut writer, "UserMetadata", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Specifies the use of SSE-KMS to encrypt delievered Inventory reports.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SSEKMS {
    /// <p>Specifies the ID of the AWS Key Management Service (KMS) master encryption key to use for encrypting Inventory reports.</p>
    pub key_id: String,
}

struct SSEKMSDeserializer;
impl SSEKMSDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SSEKMS, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SSEKMS::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "KeyId" => {
                        obj.key_id = try!(SSEKMSKeyIdDeserializer::deserialize("KeyId", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct SSEKMSSerializer;
impl SSEKMSSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SSEKMS,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("KeyId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SSEKMSKeyIdDeserializer;
impl SSEKMSKeyIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct SSEKMSKeyIdSerializer;
impl SSEKMSKeyIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Specifies the use of SSE-S3 to encrypt delievered Inventory reports.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SSES3 {}

struct SSES3Deserializer;
impl SSES3Deserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SSES3, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SSES3::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct SSES3Serializer;
impl SSES3Serializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SSES3,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectObjectContentEventStream {
    /// <p>The Continuation Event.</p>
    pub cont: Option<ContinuationEvent>,
    /// <p>The End Event.</p>
    pub end: Option<EndEvent>,
    /// <p>The Progress Event.</p>
    pub progress: Option<ProgressEvent>,
    /// <p>The Records Event.</p>
    pub records: Option<RecordsEvent>,
    /// <p>The Stats Event.</p>
    pub stats: Option<StatsEvent>,
}

struct SelectObjectContentEventStreamDeserializer;
impl SelectObjectContentEventStreamDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectObjectContentEventStream, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SelectObjectContentEventStream::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cont" => {
                        obj.cont = Some(try!(ContinuationEventDeserializer::deserialize(
                            "Cont", stack
                        )));
                    }
                    "End" => {
                        obj.end = Some(try!(EndEventDeserializer::deserialize("End", stack)));
                    }
                    "Progress" => {
                        obj.progress = Some(try!(ProgressEventDeserializer::deserialize(
                            "Progress", stack
                        )));
                    }
                    "Records" => {
                        obj.records = Some(try!(RecordsEventDeserializer::deserialize(
                            "Records", stack
                        )));
                    }
                    "Stats" => {
                        obj.stats = Some(try!(StatsEventDeserializer::deserialize("Stats", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectObjectContentOutput {
    pub payload: Option<SelectObjectContentEventStream>,
}

struct SelectObjectContentOutputDeserializer;
impl SelectObjectContentOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectObjectContentOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SelectObjectContentOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Payload" => {
                        obj.payload = Some(try!(
                            SelectObjectContentEventStreamDeserializer::deserialize(
                                "Payload", stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Request to filter the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must also specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records, and returns only records that match the specified SQL expression. You must also specify the data serialization format for the response. For more information, go to <a href="http://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectSELECTContent.html">S3Select API Documentation</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectObjectContentRequest {
    /// <p>The S3 Bucket.</p>
    pub bucket: String,
    /// <p>The expression that is used to query the object.</p>
    pub expression: String,
    /// <p>The type of the provided expression (e.g., SQL).</p>
    pub expression_type: String,
    /// <p>Describes the format of the data in the object that is being queried.</p>
    pub input_serialization: InputSerialization,
    /// <p>The Object Key.</p>
    pub key: String,
    /// <p>Describes the format of the data that you want Amazon S3 to return in response.</p>
    pub output_serialization: OutputSerialization,
    /// <p>Specifies if periodic request progress information should be enabled.</p>
    pub request_progress: Option<RequestProgress>,
    /// <p>The SSE Algorithm used to encrypt the object. For more information, go to <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html"> Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>The SSE Customer Key. For more information, go to <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html"> Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_key: Option<String>,
    /// <p>The SSE Customer Key MD5. For more information, go to <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html"> Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_key_md5: Option<String>,
}

pub struct SelectObjectContentRequestSerializer;
impl SelectObjectContentRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SelectObjectContentRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        ExpressionSerializer::serialize(&mut writer, "Expression", &obj.expression)?;
        ExpressionTypeSerializer::serialize(&mut writer, "ExpressionType", &obj.expression_type)?;
        InputSerializationSerializer::serialize(
            &mut writer,
            "InputSerialization",
            &obj.input_serialization,
        )?;
        OutputSerializationSerializer::serialize(
            &mut writer,
            "OutputSerialization",
            &obj.output_serialization,
        )?;
        if let Some(ref value) = obj.request_progress {
            &RequestProgressSerializer::serialize(&mut writer, "RequestProgress", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>Describes the parameters for Select job types.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectParameters {
    /// <p>The expression that is used to query the object.</p>
    pub expression: String,
    /// <p>The type of the provided expression (e.g., SQL).</p>
    pub expression_type: String,
    /// <p>Describes the serialization format of the object.</p>
    pub input_serialization: InputSerialization,
    /// <p>Describes how the results of the Select job are serialized.</p>
    pub output_serialization: OutputSerialization,
}

pub struct SelectParametersSerializer;
impl SelectParametersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SelectParameters,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Expression"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.expression
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("ExpressionType"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.expression_type
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        InputSerializationSerializer::serialize(
            &mut writer,
            "InputSerialization",
            &obj.input_serialization,
        )?;
        OutputSerializationSerializer::serialize(
            &mut writer,
            "OutputSerialization",
            &obj.output_serialization,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ServerSideEncryptionDeserializer;
impl ServerSideEncryptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ServerSideEncryptionSerializer;
impl ServerSideEncryptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes the default server-side encryption to apply to new objects in the bucket. If Put Object request does not specify any server-side encryption, this default encryption will be applied.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServerSideEncryptionByDefault {
    /// <p>KMS master key ID to use for the default encryption. This parameter is allowed if SSEAlgorithm is aws:kms.</p>
    pub kms_master_key_id: Option<String>,
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    pub sse_algorithm: String,
}

struct ServerSideEncryptionByDefaultDeserializer;
impl ServerSideEncryptionByDefaultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionByDefault, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ServerSideEncryptionByDefault::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "KMSMasterKeyID" => {
                        obj.kms_master_key_id = Some(try!(SSEKMSKeyIdDeserializer::deserialize(
                            "KMSMasterKeyID",
                            stack
                        )));
                    }
                    "SSEAlgorithm" => {
                        obj.sse_algorithm = try!(ServerSideEncryptionDeserializer::deserialize(
                            "SSEAlgorithm",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ServerSideEncryptionByDefaultSerializer;
impl ServerSideEncryptionByDefaultSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ServerSideEncryptionByDefault,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.kms_master_key_id {
            writer.write(xml::writer::XmlEvent::start_element("KMSMasterKeyID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("SSEAlgorithm"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.sse_algorithm
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for server-side encryption configuration rules. Currently S3 supports one rule only.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServerSideEncryptionConfiguration {
    /// <p>Container for information about a particular server-side encryption configuration rule.</p>
    pub rules: Vec<ServerSideEncryptionRule>,
}

struct ServerSideEncryptionConfigurationDeserializer;
impl ServerSideEncryptionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ServerSideEncryptionConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Rule" => {
                        obj.rules = try!(ServerSideEncryptionRulesDeserializer::deserialize(
                            "Rule", stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ServerSideEncryptionConfigurationSerializer;
impl ServerSideEncryptionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ServerSideEncryptionConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        ServerSideEncryptionRulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for information about a particular server-side encryption configuration rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServerSideEncryptionRule {
    /// <p>Describes the default server-side encryption to apply to new objects in the bucket. If Put Object request does not specify any server-side encryption, this default encryption will be applied.</p>
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
}

struct ServerSideEncryptionRuleDeserializer;
impl ServerSideEncryptionRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ServerSideEncryptionRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ApplyServerSideEncryptionByDefault" => {
                        obj.apply_server_side_encryption_by_default = Some(try!(
                            ServerSideEncryptionByDefaultDeserializer::deserialize(
                                "ApplyServerSideEncryptionByDefault",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ServerSideEncryptionRuleSerializer;
impl ServerSideEncryptionRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ServerSideEncryptionRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.apply_server_side_encryption_by_default {
            &ServerSideEncryptionByDefaultSerializer::serialize(
                &mut writer,
                "ApplyServerSideEncryptionByDefault",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ServerSideEncryptionRulesDeserializer;
impl ServerSideEncryptionRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ServerSideEncryptionRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ServerSideEncryptionRuleDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct ServerSideEncryptionRulesSerializer;
impl ServerSideEncryptionRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<ServerSideEncryptionRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            ServerSideEncryptionRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct SizeDeserializer;
impl SizeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Container for filters that define which source objects should be replicated.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SourceSelectionCriteria {
    /// <p>Container for filter information of selection of KMS Encrypted S3 objects.</p>
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

struct SourceSelectionCriteriaDeserializer;
impl SourceSelectionCriteriaDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SourceSelectionCriteria, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SourceSelectionCriteria::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "SseKmsEncryptedObjects" => {
                        obj.sse_kms_encrypted_objects =
                            Some(try!(SseKmsEncryptedObjectsDeserializer::deserialize(
                                "SseKmsEncryptedObjects",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct SourceSelectionCriteriaSerializer;
impl SourceSelectionCriteriaSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SourceSelectionCriteria,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.sse_kms_encrypted_objects {
            &SseKmsEncryptedObjectsSerializer::serialize(
                &mut writer,
                "SseKmsEncryptedObjects",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for filter information of selection of KMS Encrypted S3 objects.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SseKmsEncryptedObjects {
    /// <p>The replication for KMS encrypted S3 objects is disabled if status is not Enabled.</p>
    pub status: String,
}

struct SseKmsEncryptedObjectsDeserializer;
impl SseKmsEncryptedObjectsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SseKmsEncryptedObjects, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SseKmsEncryptedObjects::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Status" => {
                        obj.status = try!(SseKmsEncryptedObjectsStatusDeserializer::deserialize(
                            "Status", stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct SseKmsEncryptedObjectsSerializer;
impl SseKmsEncryptedObjectsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SseKmsEncryptedObjects,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Status"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.status
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SseKmsEncryptedObjectsStatusDeserializer;
impl SseKmsEncryptedObjectsStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct SseKmsEncryptedObjectsStatusSerializer;
impl SseKmsEncryptedObjectsStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct StartAfterDeserializer;
impl StartAfterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct StartAfterSerializer;
impl StartAfterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Stats {
    /// <p>Total number of uncompressed object bytes processed.</p>
    pub bytes_processed: Option<i64>,
    /// <p>Total number of bytes of records payload data returned.</p>
    pub bytes_returned: Option<i64>,
    /// <p>Total number of object bytes scanned.</p>
    pub bytes_scanned: Option<i64>,
}

struct StatsDeserializer;
impl StatsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Stats, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Stats::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "BytesProcessed" => {
                            obj.bytes_processed = Some(try!(
                                BytesProcessedDeserializer::deserialize("BytesProcessed", stack)
                            ));
                        }
                        "BytesReturned" => {
                            obj.bytes_returned = Some(try!(
                                BytesReturnedDeserializer::deserialize("BytesReturned", stack)
                            ));
                        }
                        "BytesScanned" => {
                            obj.bytes_scanned = Some(try!(BytesScannedDeserializer::deserialize(
                                "BytesScanned",
                                stack
                            )));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StatsEvent {
    /// <p>The Stats event details.</p>
    pub details: Option<Stats>,
}

struct StatsEventDeserializer;
impl StatsEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StatsEvent, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StatsEvent::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Details" => {
                        obj.details = Some(try!(StatsDeserializer::deserialize("Details", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StorageClassDeserializer;
impl StorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct StorageClassSerializer;
impl StorageClassSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StorageClassAnalysis {
    /// <p>A container used to describe how data related to the storage class analysis should be exported.</p>
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

struct StorageClassAnalysisDeserializer;
impl StorageClassAnalysisDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StorageClassAnalysis, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StorageClassAnalysis::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DataExport" => {
                        obj.data_export = Some(try!(
                            StorageClassAnalysisDataExportDeserializer::deserialize(
                                "DataExport",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct StorageClassAnalysisSerializer;
impl StorageClassAnalysisSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &StorageClassAnalysis,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.data_export {
            &StorageClassAnalysisDataExportSerializer::serialize(&mut writer, "DataExport", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StorageClassAnalysisDataExport {
    /// <p>The place to store the data for an analysis.</p>
    pub destination: AnalyticsExportDestination,
    /// <p>The version of the output schema to use when exporting data. Must be V_1.</p>
    pub output_schema_version: String,
}

struct StorageClassAnalysisDataExportDeserializer;
impl StorageClassAnalysisDataExportDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StorageClassAnalysisDataExport, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StorageClassAnalysisDataExport::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Destination" => {
                        obj.destination =
                            try!(AnalyticsExportDestinationDeserializer::deserialize(
                                "Destination",
                                stack
                            ));
                    }
                    "OutputSchemaVersion" => {
                        obj.output_schema_version =
                            try!(StorageClassAnalysisSchemaVersionDeserializer::deserialize(
                                "OutputSchemaVersion",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct StorageClassAnalysisDataExportSerializer;
impl StorageClassAnalysisDataExportSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &StorageClassAnalysisDataExport,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        AnalyticsExportDestinationSerializer::serialize(
            &mut writer,
            "Destination",
            &obj.destination,
        )?;
        writer.write(xml::writer::XmlEvent::start_element("OutputSchemaVersion"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.output_schema_version
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct StorageClassAnalysisSchemaVersionDeserializer;
impl StorageClassAnalysisSchemaVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct StorageClassAnalysisSchemaVersionSerializer;
impl StorageClassAnalysisSchemaVersionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SuffixDeserializer;
impl SuffixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct SuffixSerializer;
impl SuffixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p>Name of the tag.</p>
    pub key: String,
    /// <p>Value of the tag.</p>
    pub value: String,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tag, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Tag::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Key" => {
                        obj.key = try!(ObjectKeyDeserializer::deserialize("Key", stack));
                    }
                    "Value" => {
                        obj.value = try!(ValueDeserializer::deserialize("Value", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TagSerializer;
impl TagSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Tag,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Value"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.value
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TagSetDeserializer;
impl TagSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Tag" {
                        obj.push(try!(TagDeserializer::deserialize("Tag", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

pub struct TagSetSerializer;
impl TagSetSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Tag>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            TagSerializer::serialize(writer, "Tag", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tagging {
    pub tag_set: Vec<Tag>,
}

pub struct TaggingSerializer;
impl TaggingSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Tagging,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        TagSetSerializer::serialize(&mut writer, "TagSet", &obj.tag_set)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TargetBucketDeserializer;
impl TargetBucketDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TargetBucketSerializer;
impl TargetBucketSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TargetGrant {
    pub grantee: Option<Grantee>,
    /// <p>Logging permissions assigned to the Grantee for the bucket.</p>
    pub permission: Option<String>,
}

struct TargetGrantDeserializer;
impl TargetGrantDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGrant, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TargetGrant::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Grantee" => {
                        obj.grantee =
                            Some(try!(GranteeDeserializer::deserialize("Grantee", stack)));
                    }
                    "Permission" => {
                        obj.permission = Some(try!(BucketLogsPermissionDeserializer::deserialize(
                            "Permission",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TargetGrantSerializer;
impl TargetGrantSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TargetGrant,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.grantee {
            &GranteeSerializer::serialize(&mut writer, "Grantee", value)?;
        }
        if let Some(ref value) = obj.permission {
            writer.write(xml::writer::XmlEvent::start_element("Permission"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TargetGrantsDeserializer;
impl TargetGrantsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetGrant>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Grant" {
                        obj.push(try!(TargetGrantDeserializer::deserialize("Grant", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

pub struct TargetGrantsSerializer;
impl TargetGrantsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<TargetGrant>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            TargetGrantSerializer::serialize(writer, "Grant", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

struct TargetPrefixDeserializer;
impl TargetPrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TargetPrefixSerializer;
impl TargetPrefixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct TierSerializer;
impl TierSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TokenDeserializer;
impl TokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TokenSerializer;
impl TokenSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TopicArnDeserializer;
impl TopicArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TopicArnSerializer;
impl TopicArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for specifying the configuration when you want Amazon S3 to publish events to an Amazon Simple Notification Service (Amazon SNS) topic.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TopicConfiguration {
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>Amazon SNS topic ARN to which Amazon S3 will publish a message when it detects events of specified type.</p>
    pub topic_arn: String,
}

struct TopicConfigurationDeserializer;
impl TopicConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TopicConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TopicConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Event" => {
                        obj.events = try!(EventListDeserializer::deserialize("Event", stack));
                    }
                    "Filter" => {
                        obj.filter = Some(try!(
                            NotificationConfigurationFilterDeserializer::deserialize(
                                "Filter", stack
                            )
                        ));
                    }
                    "Id" => {
                        obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id", stack)));
                    }
                    "Topic" => {
                        obj.topic_arn = try!(TopicArnDeserializer::deserialize("Topic", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TopicConfigurationSerializer;
impl TopicConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TopicConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Topic"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.topic_arn
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TopicConfigurationDeprecated {
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    /// <p>Amazon SNS topic to which Amazon S3 will publish a message to report the specified events for the bucket.</p>
    pub topic: Option<String>,
}

struct TopicConfigurationDeprecatedDeserializer;
impl TopicConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TopicConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TopicConfigurationDeprecated::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Event" => {
                        obj.events = Some(try!(EventListDeserializer::deserialize("Event", stack)));
                    }
                    "Id" => {
                        obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id", stack)));
                    }
                    "Topic" => {
                        obj.topic = Some(try!(TopicArnDeserializer::deserialize("Topic", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TopicConfigurationDeprecatedSerializer;
impl TopicConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TopicConfigurationDeprecated,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.topic {
            writer.write(xml::writer::XmlEvent::start_element("Topic"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TopicConfigurationListDeserializer;
impl TopicConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TopicConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(TopicConfigurationDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct TopicConfigurationListSerializer;
impl TopicConfigurationListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<TopicConfiguration>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            TopicConfigurationSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transition {
    /// <p>Indicates at what date the object is to be moved or deleted. Should be in GMT ISO 8601 Format.</p>
    pub date: Option<String>,
    /// <p>Indicates the lifetime, in days, of the objects that are subject to the rule. The value must be a non-zero positive integer.</p>
    pub days: Option<i64>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
}

struct TransitionDeserializer;
impl TransitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Transition, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Transition::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Date" => {
                        obj.date = Some(try!(DateDeserializer::deserialize("Date", stack)));
                    }
                    "Days" => {
                        obj.days = Some(try!(DaysDeserializer::deserialize("Days", stack)));
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(try!(
                            TransitionStorageClassDeserializer::deserialize("StorageClass", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TransitionSerializer;
impl TransitionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Transition,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.date {
            writer.write(xml::writer::XmlEvent::start_element("Date"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.storage_class {
            writer.write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TransitionListDeserializer;
impl TransitionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Transition>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(TransitionDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct TransitionListSerializer;
impl TransitionListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Transition>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            TransitionSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct TransitionStorageClassDeserializer;
impl TransitionStorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TransitionStorageClassSerializer;
impl TransitionStorageClassSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TypeDeserializer;
impl TypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct TypeSerializer;
impl TypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct URIDeserializer;
impl URIDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct URISerializer;
impl URISerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct UploadIdMarkerDeserializer;
impl UploadIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct UploadIdMarkerSerializer;
impl UploadIdMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UploadPartCopyOutput {
    pub copy_part_result: Option<CopyPartResult>,
    /// <p>The version of the source object that was copied, if you have enabled versioning on the source bucket.</p>
    pub copy_source_version_id: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
}

struct UploadPartCopyOutputDeserializer;
impl UploadPartCopyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartCopyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UploadPartCopyOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CopyPartResult" => {
                        obj.copy_part_result = Some(try!(CopyPartResultDeserializer::deserialize(
                            "CopyPartResult",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UploadPartCopyRequest {
    pub bucket: String,
    /// <p>The name of the source bucket and key name of the source object, separated by a slash (/). Must be URL-encoded.</p>
    pub copy_source: String,
    /// <p>Copies the object if its entity tag (ETag) matches the specified tag.</p>
    pub copy_source_if_match: Option<String>,
    /// <p>Copies the object if it has been modified since the specified time.</p>
    pub copy_source_if_modified_since: Option<String>,
    /// <p>Copies the object if its entity tag (ETag) is different than the specified ETag.</p>
    pub copy_source_if_none_match: Option<String>,
    /// <p>Copies the object if it hasn't been modified since the specified time.</p>
    pub copy_source_if_unmodified_since: Option<String>,
    /// <p>The range of bytes to copy from the source object. The range value must use the form bytes=first-last, where the first and last are the zero-based byte offsets to copy. For example, bytes=0-9 indicates that you want to copy the first ten bytes of the source. You can copy a range only if the source object is greater than 5 GB.</p>
    pub copy_source_range: Option<String>,
    /// <p>Specifies the algorithm to use when decrypting the source object (e.g., AES256).</p>
    pub copy_source_sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.</p>
    pub copy_source_sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub copy_source_sse_customer_key_md5: Option<String>,
    pub key: String,
    /// <p>Part number of part being copied. This is a positive integer between 1 and 10,000.</p>
    pub part_number: i64,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header. This must be the same encryption key specified in the initiate multipart upload request.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Upload ID identifying the multipart upload whose part is being copied.</p>
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UploadPartOutput {
    /// <p>Entity tag for the uploaded object.</p>
    pub e_tag: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
}

struct UploadPartOutputDeserializer;
impl UploadPartOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = UploadPartOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug)]
pub struct UploadPartRequest {
    /// <p>Object data.</p>
    pub body: Option<StreamingBody>,
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: String,
    /// <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically.</p>
    pub content_length: Option<i64>,
    /// <p>The base64-encoded 128-bit MD5 digest of the part data.</p>
    pub content_md5: Option<String>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: String,
    /// <p>Part number of part being uploaded. This is a positive integer between 1 and 10,000.</p>
    pub part_number: i64,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header. This must be the same encryption key specified in the initiate multipart upload request.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Upload ID identifying the multipart upload whose part is being uploaded.</p>
    pub upload_id: String,
}

pub struct UserMetadataSerializer;
impl UserMetadataSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<MetadataEntry>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            MetadataEntrySerializer::serialize(writer, "MetadataEntry", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

struct ValueDeserializer;
impl ValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ValueSerializer;
impl ValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct VersionIdMarkerDeserializer;
impl VersionIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct VersionIdMarkerSerializer;
impl VersionIdMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct VersioningConfiguration {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<String>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<String>,
}

pub struct VersioningConfigurationSerializer;
impl VersioningConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &VersioningConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.mfa_delete {
            writer.write(xml::writer::XmlEvent::start_element("MfaDelete"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.status {
            writer.write(xml::writer::XmlEvent::start_element("Status"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct WebsiteConfiguration {
    pub error_document: Option<ErrorDocument>,
    pub index_document: Option<IndexDocument>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub routing_rules: Option<Vec<RoutingRule>>,
}

pub struct WebsiteConfigurationSerializer;
impl WebsiteConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &WebsiteConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.error_document {
            &ErrorDocumentSerializer::serialize(&mut writer, "ErrorDocument", value)?;
        }
        if let Some(ref value) = obj.index_document {
            &IndexDocumentSerializer::serialize(&mut writer, "IndexDocument", value)?;
        }
        if let Some(ref value) = obj.redirect_all_requests_to {
            &RedirectAllRequestsToSerializer::serialize(
                &mut writer,
                "RedirectAllRequestsTo",
                value,
            )?;
        }
        if let Some(ref value) = obj.routing_rules {
            &RoutingRulesSerializer::serialize(&mut writer, "RoutingRules", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// Errors returned by AbortMultipartUpload
#[derive(Debug, PartialEq)]
pub enum AbortMultipartUploadError {
    /// <p>The specified multipart upload does not exist.</p>
    NoSuchUpload(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AbortMultipartUploadError {
    pub fn from_body(body: &str) -> AbortMultipartUploadError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "NoSuchUpload" => {
                    AbortMultipartUploadError::NoSuchUpload(String::from(parsed_error.message))
                }
                _ => AbortMultipartUploadError::Unknown(String::from(body)),
            },
            Err(_) => AbortMultipartUploadError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for AbortMultipartUploadError {
    fn from(err: XmlParseError) -> AbortMultipartUploadError {
        let XmlParseError(message) = err;
        AbortMultipartUploadError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AbortMultipartUploadError {
    fn from(err: CredentialsError) -> AbortMultipartUploadError {
        AbortMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AbortMultipartUploadError {
    fn from(err: HttpDispatchError) -> AbortMultipartUploadError {
        AbortMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for AbortMultipartUploadError {
    fn from(err: io::Error) -> AbortMultipartUploadError {
        AbortMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AbortMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AbortMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            AbortMultipartUploadError::NoSuchUpload(ref cause) => cause,
            AbortMultipartUploadError::Validation(ref cause) => cause,
            AbortMultipartUploadError::Credentials(ref err) => err.description(),
            AbortMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AbortMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CompleteMultipartUpload
#[derive(Debug, PartialEq)]
pub enum CompleteMultipartUploadError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CompleteMultipartUploadError {
    pub fn from_body(body: &str) -> CompleteMultipartUploadError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => CompleteMultipartUploadError::Unknown(String::from(body)),
            },
            Err(_) => CompleteMultipartUploadError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CompleteMultipartUploadError {
    fn from(err: XmlParseError) -> CompleteMultipartUploadError {
        let XmlParseError(message) = err;
        CompleteMultipartUploadError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CompleteMultipartUploadError {
    fn from(err: CredentialsError) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CompleteMultipartUploadError {
    fn from(err: HttpDispatchError) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for CompleteMultipartUploadError {
    fn from(err: io::Error) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CompleteMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompleteMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            CompleteMultipartUploadError::Validation(ref cause) => cause,
            CompleteMultipartUploadError::Credentials(ref err) => err.description(),
            CompleteMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CompleteMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CopyObject
#[derive(Debug, PartialEq)]
pub enum CopyObjectError {
    /// <p>The source object of the COPY operation is not in the active tier and is only stored in Amazon Glacier.</p>
    ObjectNotInActiveTierError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CopyObjectError {
    pub fn from_body(body: &str) -> CopyObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ObjectNotInActiveTierError" => {
                    CopyObjectError::ObjectNotInActiveTierError(String::from(parsed_error.message))
                }
                _ => CopyObjectError::Unknown(String::from(body)),
            },
            Err(_) => CopyObjectError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CopyObjectError {
    fn from(err: XmlParseError) -> CopyObjectError {
        let XmlParseError(message) = err;
        CopyObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CopyObjectError {
    fn from(err: CredentialsError) -> CopyObjectError {
        CopyObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopyObjectError {
    fn from(err: HttpDispatchError) -> CopyObjectError {
        CopyObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CopyObjectError {
    fn from(err: io::Error) -> CopyObjectError {
        CopyObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CopyObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyObjectError {
    fn description(&self) -> &str {
        match *self {
            CopyObjectError::ObjectNotInActiveTierError(ref cause) => cause,
            CopyObjectError::Validation(ref cause) => cause,
            CopyObjectError::Credentials(ref err) => err.description(),
            CopyObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CopyObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBucket
#[derive(Debug, PartialEq)]
pub enum CreateBucketError {
    /// <p>The requested bucket name is not available. The bucket namespace is shared by all users of the system. Please select a different name and try again.</p>
    BucketAlreadyExists(String),

    BucketAlreadyOwnedByYou(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateBucketError {
    pub fn from_body(body: &str) -> CreateBucketError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BucketAlreadyExists" => {
                    CreateBucketError::BucketAlreadyExists(String::from(parsed_error.message))
                }
                "BucketAlreadyOwnedByYou" => {
                    CreateBucketError::BucketAlreadyOwnedByYou(String::from(parsed_error.message))
                }
                _ => CreateBucketError::Unknown(String::from(body)),
            },
            Err(_) => CreateBucketError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateBucketError {
    fn from(err: XmlParseError) -> CreateBucketError {
        let XmlParseError(message) = err;
        CreateBucketError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateBucketError {
    fn from(err: CredentialsError) -> CreateBucketError {
        CreateBucketError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBucketError {
    fn from(err: HttpDispatchError) -> CreateBucketError {
        CreateBucketError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBucketError {
    fn from(err: io::Error) -> CreateBucketError {
        CreateBucketError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBucketError {
    fn description(&self) -> &str {
        match *self {
            CreateBucketError::BucketAlreadyExists(ref cause) => cause,
            CreateBucketError::BucketAlreadyOwnedByYou(ref cause) => cause,
            CreateBucketError::Validation(ref cause) => cause,
            CreateBucketError::Credentials(ref err) => err.description(),
            CreateBucketError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBucketError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMultipartUpload
#[derive(Debug, PartialEq)]
pub enum CreateMultipartUploadError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateMultipartUploadError {
    pub fn from_body(body: &str) -> CreateMultipartUploadError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => CreateMultipartUploadError::Unknown(String::from(body)),
            },
            Err(_) => CreateMultipartUploadError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateMultipartUploadError {
    fn from(err: XmlParseError) -> CreateMultipartUploadError {
        let XmlParseError(message) = err;
        CreateMultipartUploadError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateMultipartUploadError {
    fn from(err: CredentialsError) -> CreateMultipartUploadError {
        CreateMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMultipartUploadError {
    fn from(err: HttpDispatchError) -> CreateMultipartUploadError {
        CreateMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateMultipartUploadError {
    fn from(err: io::Error) -> CreateMultipartUploadError {
        CreateMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            CreateMultipartUploadError::Validation(ref cause) => cause,
            CreateMultipartUploadError::Credentials(ref err) => err.description(),
            CreateMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucket
#[derive(Debug, PartialEq)]
pub enum DeleteBucketError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketError {
    pub fn from_body(body: &str) -> DeleteBucketError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketError {
    fn from(err: XmlParseError) -> DeleteBucketError {
        let XmlParseError(message) = err;
        DeleteBucketError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketError {
    fn from(err: CredentialsError) -> DeleteBucketError {
        DeleteBucketError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketError {
    fn from(err: HttpDispatchError) -> DeleteBucketError {
        DeleteBucketError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketError {
    fn from(err: io::Error) -> DeleteBucketError {
        DeleteBucketError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketError::Validation(ref cause) => cause,
            DeleteBucketError::Credentials(ref err) => err.description(),
            DeleteBucketError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBucketError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketAnalyticsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketAnalyticsConfigurationError {
    pub fn from_body(body: &str) -> DeleteBucketAnalyticsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketAnalyticsConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketAnalyticsConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketAnalyticsConfigurationError {
    fn from(err: XmlParseError) -> DeleteBucketAnalyticsConfigurationError {
        let XmlParseError(message) = err;
        DeleteBucketAnalyticsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketAnalyticsConfigurationError {
    fn from(err: CredentialsError) -> DeleteBucketAnalyticsConfigurationError {
        DeleteBucketAnalyticsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketAnalyticsConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteBucketAnalyticsConfigurationError {
        DeleteBucketAnalyticsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketAnalyticsConfigurationError {
    fn from(err: io::Error) -> DeleteBucketAnalyticsConfigurationError {
        DeleteBucketAnalyticsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketAnalyticsConfigurationError::Validation(ref cause) => cause,
            DeleteBucketAnalyticsConfigurationError::Credentials(ref err) => err.description(),
            DeleteBucketAnalyticsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketAnalyticsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketCors
#[derive(Debug, PartialEq)]
pub enum DeleteBucketCorsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketCorsError {
    pub fn from_body(body: &str) -> DeleteBucketCorsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketCorsError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketCorsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketCorsError {
    fn from(err: XmlParseError) -> DeleteBucketCorsError {
        let XmlParseError(message) = err;
        DeleteBucketCorsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketCorsError {
    fn from(err: CredentialsError) -> DeleteBucketCorsError {
        DeleteBucketCorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketCorsError {
    fn from(err: HttpDispatchError) -> DeleteBucketCorsError {
        DeleteBucketCorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketCorsError {
    fn from(err: io::Error) -> DeleteBucketCorsError {
        DeleteBucketCorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketCorsError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketCorsError::Validation(ref cause) => cause,
            DeleteBucketCorsError::Credentials(ref err) => err.description(),
            DeleteBucketCorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBucketCorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketEncryption
#[derive(Debug, PartialEq)]
pub enum DeleteBucketEncryptionError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketEncryptionError {
    pub fn from_body(body: &str) -> DeleteBucketEncryptionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketEncryptionError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketEncryptionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketEncryptionError {
    fn from(err: XmlParseError) -> DeleteBucketEncryptionError {
        let XmlParseError(message) = err;
        DeleteBucketEncryptionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketEncryptionError {
    fn from(err: CredentialsError) -> DeleteBucketEncryptionError {
        DeleteBucketEncryptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketEncryptionError {
    fn from(err: HttpDispatchError) -> DeleteBucketEncryptionError {
        DeleteBucketEncryptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketEncryptionError {
    fn from(err: io::Error) -> DeleteBucketEncryptionError {
        DeleteBucketEncryptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketEncryptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketEncryptionError::Validation(ref cause) => cause,
            DeleteBucketEncryptionError::Credentials(ref err) => err.description(),
            DeleteBucketEncryptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketEncryptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketInventoryConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketInventoryConfigurationError {
    pub fn from_body(body: &str) -> DeleteBucketInventoryConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketInventoryConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketInventoryConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketInventoryConfigurationError {
    fn from(err: XmlParseError) -> DeleteBucketInventoryConfigurationError {
        let XmlParseError(message) = err;
        DeleteBucketInventoryConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketInventoryConfigurationError {
    fn from(err: CredentialsError) -> DeleteBucketInventoryConfigurationError {
        DeleteBucketInventoryConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketInventoryConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteBucketInventoryConfigurationError {
        DeleteBucketInventoryConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketInventoryConfigurationError {
    fn from(err: io::Error) -> DeleteBucketInventoryConfigurationError {
        DeleteBucketInventoryConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketInventoryConfigurationError::Validation(ref cause) => cause,
            DeleteBucketInventoryConfigurationError::Credentials(ref err) => err.description(),
            DeleteBucketInventoryConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketInventoryConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum DeleteBucketLifecycleError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketLifecycleError {
    pub fn from_body(body: &str) -> DeleteBucketLifecycleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketLifecycleError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketLifecycleError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketLifecycleError {
    fn from(err: XmlParseError) -> DeleteBucketLifecycleError {
        let XmlParseError(message) = err;
        DeleteBucketLifecycleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketLifecycleError {
    fn from(err: CredentialsError) -> DeleteBucketLifecycleError {
        DeleteBucketLifecycleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketLifecycleError {
    fn from(err: HttpDispatchError) -> DeleteBucketLifecycleError {
        DeleteBucketLifecycleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketLifecycleError {
    fn from(err: io::Error) -> DeleteBucketLifecycleError {
        DeleteBucketLifecycleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketLifecycleError::Validation(ref cause) => cause,
            DeleteBucketLifecycleError::Credentials(ref err) => err.description(),
            DeleteBucketLifecycleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketLifecycleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketMetricsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketMetricsConfigurationError {
    pub fn from_body(body: &str) -> DeleteBucketMetricsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketMetricsConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketMetricsConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketMetricsConfigurationError {
    fn from(err: XmlParseError) -> DeleteBucketMetricsConfigurationError {
        let XmlParseError(message) = err;
        DeleteBucketMetricsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketMetricsConfigurationError {
    fn from(err: CredentialsError) -> DeleteBucketMetricsConfigurationError {
        DeleteBucketMetricsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketMetricsConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteBucketMetricsConfigurationError {
        DeleteBucketMetricsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketMetricsConfigurationError {
    fn from(err: io::Error) -> DeleteBucketMetricsConfigurationError {
        DeleteBucketMetricsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketMetricsConfigurationError::Validation(ref cause) => cause,
            DeleteBucketMetricsConfigurationError::Credentials(ref err) => err.description(),
            DeleteBucketMetricsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketMetricsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteBucketPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketPolicyError {
    pub fn from_body(body: &str) -> DeleteBucketPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketPolicyError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketPolicyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketPolicyError {
    fn from(err: XmlParseError) -> DeleteBucketPolicyError {
        let XmlParseError(message) = err;
        DeleteBucketPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketPolicyError {
    fn from(err: CredentialsError) -> DeleteBucketPolicyError {
        DeleteBucketPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketPolicyError {
    fn from(err: HttpDispatchError) -> DeleteBucketPolicyError {
        DeleteBucketPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketPolicyError {
    fn from(err: io::Error) -> DeleteBucketPolicyError {
        DeleteBucketPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketPolicyError::Validation(ref cause) => cause,
            DeleteBucketPolicyError::Credentials(ref err) => err.description(),
            DeleteBucketPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketReplication
#[derive(Debug, PartialEq)]
pub enum DeleteBucketReplicationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketReplicationError {
    pub fn from_body(body: &str) -> DeleteBucketReplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketReplicationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketReplicationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketReplicationError {
    fn from(err: XmlParseError) -> DeleteBucketReplicationError {
        let XmlParseError(message) = err;
        DeleteBucketReplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketReplicationError {
    fn from(err: CredentialsError) -> DeleteBucketReplicationError {
        DeleteBucketReplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketReplicationError {
    fn from(err: HttpDispatchError) -> DeleteBucketReplicationError {
        DeleteBucketReplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketReplicationError {
    fn from(err: io::Error) -> DeleteBucketReplicationError {
        DeleteBucketReplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketReplicationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketReplicationError::Validation(ref cause) => cause,
            DeleteBucketReplicationError::Credentials(ref err) => err.description(),
            DeleteBucketReplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketReplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketTagging
#[derive(Debug, PartialEq)]
pub enum DeleteBucketTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketTaggingError {
    pub fn from_body(body: &str) -> DeleteBucketTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketTaggingError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketTaggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketTaggingError {
    fn from(err: XmlParseError) -> DeleteBucketTaggingError {
        let XmlParseError(message) = err;
        DeleteBucketTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketTaggingError {
    fn from(err: CredentialsError) -> DeleteBucketTaggingError {
        DeleteBucketTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketTaggingError {
    fn from(err: HttpDispatchError) -> DeleteBucketTaggingError {
        DeleteBucketTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketTaggingError {
    fn from(err: io::Error) -> DeleteBucketTaggingError {
        DeleteBucketTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketTaggingError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketTaggingError::Validation(ref cause) => cause,
            DeleteBucketTaggingError::Credentials(ref err) => err.description(),
            DeleteBucketTaggingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketWebsite
#[derive(Debug, PartialEq)]
pub enum DeleteBucketWebsiteError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBucketWebsiteError {
    pub fn from_body(body: &str) -> DeleteBucketWebsiteError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteBucketWebsiteError::Unknown(String::from(body)),
            },
            Err(_) => DeleteBucketWebsiteError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteBucketWebsiteError {
    fn from(err: XmlParseError) -> DeleteBucketWebsiteError {
        let XmlParseError(message) = err;
        DeleteBucketWebsiteError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketWebsiteError {
    fn from(err: CredentialsError) -> DeleteBucketWebsiteError {
        DeleteBucketWebsiteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketWebsiteError {
    fn from(err: HttpDispatchError) -> DeleteBucketWebsiteError {
        DeleteBucketWebsiteError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketWebsiteError {
    fn from(err: io::Error) -> DeleteBucketWebsiteError {
        DeleteBucketWebsiteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketWebsiteError::Validation(ref cause) => cause,
            DeleteBucketWebsiteError::Credentials(ref err) => err.description(),
            DeleteBucketWebsiteError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketWebsiteError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteObject
#[derive(Debug, PartialEq)]
pub enum DeleteObjectError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteObjectError {
    pub fn from_body(body: &str) -> DeleteObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteObjectError::Unknown(String::from(body)),
            },
            Err(_) => DeleteObjectError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteObjectError {
    fn from(err: XmlParseError) -> DeleteObjectError {
        let XmlParseError(message) = err;
        DeleteObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteObjectError {
    fn from(err: CredentialsError) -> DeleteObjectError {
        DeleteObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectError {
    fn from(err: HttpDispatchError) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectError {
    fn from(err: io::Error) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectError::Validation(ref cause) => cause,
            DeleteObjectError::Credentials(ref err) => err.description(),
            DeleteObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteObjectTagging
#[derive(Debug, PartialEq)]
pub enum DeleteObjectTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteObjectTaggingError {
    pub fn from_body(body: &str) -> DeleteObjectTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteObjectTaggingError::Unknown(String::from(body)),
            },
            Err(_) => DeleteObjectTaggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteObjectTaggingError {
    fn from(err: XmlParseError) -> DeleteObjectTaggingError {
        let XmlParseError(message) = err;
        DeleteObjectTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteObjectTaggingError {
    fn from(err: CredentialsError) -> DeleteObjectTaggingError {
        DeleteObjectTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectTaggingError {
    fn from(err: HttpDispatchError) -> DeleteObjectTaggingError {
        DeleteObjectTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectTaggingError {
    fn from(err: io::Error) -> DeleteObjectTaggingError {
        DeleteObjectTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectTaggingError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectTaggingError::Validation(ref cause) => cause,
            DeleteObjectTaggingError::Credentials(ref err) => err.description(),
            DeleteObjectTaggingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteObjectTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteObjects
#[derive(Debug, PartialEq)]
pub enum DeleteObjectsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteObjectsError {
    pub fn from_body(body: &str) -> DeleteObjectsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteObjectsError::Unknown(String::from(body)),
            },
            Err(_) => DeleteObjectsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteObjectsError {
    fn from(err: XmlParseError) -> DeleteObjectsError {
        let XmlParseError(message) = err;
        DeleteObjectsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteObjectsError {
    fn from(err: CredentialsError) -> DeleteObjectsError {
        DeleteObjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectsError {
    fn from(err: HttpDispatchError) -> DeleteObjectsError {
        DeleteObjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectsError {
    fn from(err: io::Error) -> DeleteObjectsError {
        DeleteObjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectsError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectsError::Validation(ref cause) => cause,
            DeleteObjectsError::Credentials(ref err) => err.description(),
            DeleteObjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteObjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketAccelerateConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketAccelerateConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketAccelerateConfigurationError {
    pub fn from_body(body: &str) -> GetBucketAccelerateConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketAccelerateConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketAccelerateConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketAccelerateConfigurationError {
    fn from(err: XmlParseError) -> GetBucketAccelerateConfigurationError {
        let XmlParseError(message) = err;
        GetBucketAccelerateConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketAccelerateConfigurationError {
    fn from(err: CredentialsError) -> GetBucketAccelerateConfigurationError {
        GetBucketAccelerateConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketAccelerateConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketAccelerateConfigurationError {
        GetBucketAccelerateConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketAccelerateConfigurationError {
    fn from(err: io::Error) -> GetBucketAccelerateConfigurationError {
        GetBucketAccelerateConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketAccelerateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAccelerateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketAccelerateConfigurationError::Validation(ref cause) => cause,
            GetBucketAccelerateConfigurationError::Credentials(ref err) => err.description(),
            GetBucketAccelerateConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketAccelerateConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketAcl
#[derive(Debug, PartialEq)]
pub enum GetBucketAclError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketAclError {
    pub fn from_body(body: &str) -> GetBucketAclError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketAclError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketAclError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketAclError {
    fn from(err: XmlParseError) -> GetBucketAclError {
        let XmlParseError(message) = err;
        GetBucketAclError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketAclError {
    fn from(err: CredentialsError) -> GetBucketAclError {
        GetBucketAclError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketAclError {
    fn from(err: HttpDispatchError) -> GetBucketAclError {
        GetBucketAclError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketAclError {
    fn from(err: io::Error) -> GetBucketAclError {
        GetBucketAclError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAclError {
    fn description(&self) -> &str {
        match *self {
            GetBucketAclError::Validation(ref cause) => cause,
            GetBucketAclError::Credentials(ref err) => err.description(),
            GetBucketAclError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketAclError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketAnalyticsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketAnalyticsConfigurationError {
    pub fn from_body(body: &str) -> GetBucketAnalyticsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketAnalyticsConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketAnalyticsConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketAnalyticsConfigurationError {
    fn from(err: XmlParseError) -> GetBucketAnalyticsConfigurationError {
        let XmlParseError(message) = err;
        GetBucketAnalyticsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketAnalyticsConfigurationError {
    fn from(err: CredentialsError) -> GetBucketAnalyticsConfigurationError {
        GetBucketAnalyticsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketAnalyticsConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketAnalyticsConfigurationError {
        GetBucketAnalyticsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketAnalyticsConfigurationError {
    fn from(err: io::Error) -> GetBucketAnalyticsConfigurationError {
        GetBucketAnalyticsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketAnalyticsConfigurationError::Validation(ref cause) => cause,
            GetBucketAnalyticsConfigurationError::Credentials(ref err) => err.description(),
            GetBucketAnalyticsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketAnalyticsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketCors
#[derive(Debug, PartialEq)]
pub enum GetBucketCorsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketCorsError {
    pub fn from_body(body: &str) -> GetBucketCorsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketCorsError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketCorsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketCorsError {
    fn from(err: XmlParseError) -> GetBucketCorsError {
        let XmlParseError(message) = err;
        GetBucketCorsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketCorsError {
    fn from(err: CredentialsError) -> GetBucketCorsError {
        GetBucketCorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketCorsError {
    fn from(err: HttpDispatchError) -> GetBucketCorsError {
        GetBucketCorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketCorsError {
    fn from(err: io::Error) -> GetBucketCorsError {
        GetBucketCorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketCorsError {
    fn description(&self) -> &str {
        match *self {
            GetBucketCorsError::Validation(ref cause) => cause,
            GetBucketCorsError::Credentials(ref err) => err.description(),
            GetBucketCorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketCorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketEncryption
#[derive(Debug, PartialEq)]
pub enum GetBucketEncryptionError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketEncryptionError {
    pub fn from_body(body: &str) -> GetBucketEncryptionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketEncryptionError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketEncryptionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketEncryptionError {
    fn from(err: XmlParseError) -> GetBucketEncryptionError {
        let XmlParseError(message) = err;
        GetBucketEncryptionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketEncryptionError {
    fn from(err: CredentialsError) -> GetBucketEncryptionError {
        GetBucketEncryptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketEncryptionError {
    fn from(err: HttpDispatchError) -> GetBucketEncryptionError {
        GetBucketEncryptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketEncryptionError {
    fn from(err: io::Error) -> GetBucketEncryptionError {
        GetBucketEncryptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketEncryptionError {
    fn description(&self) -> &str {
        match *self {
            GetBucketEncryptionError::Validation(ref cause) => cause,
            GetBucketEncryptionError::Credentials(ref err) => err.description(),
            GetBucketEncryptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketEncryptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketInventoryConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketInventoryConfigurationError {
    pub fn from_body(body: &str) -> GetBucketInventoryConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketInventoryConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketInventoryConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketInventoryConfigurationError {
    fn from(err: XmlParseError) -> GetBucketInventoryConfigurationError {
        let XmlParseError(message) = err;
        GetBucketInventoryConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketInventoryConfigurationError {
    fn from(err: CredentialsError) -> GetBucketInventoryConfigurationError {
        GetBucketInventoryConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketInventoryConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketInventoryConfigurationError {
        GetBucketInventoryConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketInventoryConfigurationError {
    fn from(err: io::Error) -> GetBucketInventoryConfigurationError {
        GetBucketInventoryConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketInventoryConfigurationError::Validation(ref cause) => cause,
            GetBucketInventoryConfigurationError::Credentials(ref err) => err.description(),
            GetBucketInventoryConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketInventoryConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum GetBucketLifecycleError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketLifecycleError {
    pub fn from_body(body: &str) -> GetBucketLifecycleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketLifecycleError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketLifecycleError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketLifecycleError {
    fn from(err: XmlParseError) -> GetBucketLifecycleError {
        let XmlParseError(message) = err;
        GetBucketLifecycleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketLifecycleError {
    fn from(err: CredentialsError) -> GetBucketLifecycleError {
        GetBucketLifecycleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketLifecycleError {
    fn from(err: HttpDispatchError) -> GetBucketLifecycleError {
        GetBucketLifecycleError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketLifecycleError {
    fn from(err: io::Error) -> GetBucketLifecycleError {
        GetBucketLifecycleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {
            GetBucketLifecycleError::Validation(ref cause) => cause,
            GetBucketLifecycleError::Credentials(ref err) => err.description(),
            GetBucketLifecycleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketLifecycleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketLifecycleConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketLifecycleConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketLifecycleConfigurationError {
    pub fn from_body(body: &str) -> GetBucketLifecycleConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketLifecycleConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketLifecycleConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketLifecycleConfigurationError {
    fn from(err: XmlParseError) -> GetBucketLifecycleConfigurationError {
        let XmlParseError(message) = err;
        GetBucketLifecycleConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketLifecycleConfigurationError {
    fn from(err: CredentialsError) -> GetBucketLifecycleConfigurationError {
        GetBucketLifecycleConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketLifecycleConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketLifecycleConfigurationError {
        GetBucketLifecycleConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketLifecycleConfigurationError {
    fn from(err: io::Error) -> GetBucketLifecycleConfigurationError {
        GetBucketLifecycleConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketLifecycleConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLifecycleConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketLifecycleConfigurationError::Validation(ref cause) => cause,
            GetBucketLifecycleConfigurationError::Credentials(ref err) => err.description(),
            GetBucketLifecycleConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketLifecycleConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketLocation
#[derive(Debug, PartialEq)]
pub enum GetBucketLocationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketLocationError {
    pub fn from_body(body: &str) -> GetBucketLocationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketLocationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketLocationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketLocationError {
    fn from(err: XmlParseError) -> GetBucketLocationError {
        let XmlParseError(message) = err;
        GetBucketLocationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketLocationError {
    fn from(err: CredentialsError) -> GetBucketLocationError {
        GetBucketLocationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketLocationError {
    fn from(err: HttpDispatchError) -> GetBucketLocationError {
        GetBucketLocationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketLocationError {
    fn from(err: io::Error) -> GetBucketLocationError {
        GetBucketLocationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketLocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLocationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketLocationError::Validation(ref cause) => cause,
            GetBucketLocationError::Credentials(ref err) => err.description(),
            GetBucketLocationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketLocationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketLogging
#[derive(Debug, PartialEq)]
pub enum GetBucketLoggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketLoggingError {
    pub fn from_body(body: &str) -> GetBucketLoggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketLoggingError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketLoggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketLoggingError {
    fn from(err: XmlParseError) -> GetBucketLoggingError {
        let XmlParseError(message) = err;
        GetBucketLoggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketLoggingError {
    fn from(err: CredentialsError) -> GetBucketLoggingError {
        GetBucketLoggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketLoggingError {
    fn from(err: HttpDispatchError) -> GetBucketLoggingError {
        GetBucketLoggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketLoggingError {
    fn from(err: io::Error) -> GetBucketLoggingError {
        GetBucketLoggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLoggingError {
    fn description(&self) -> &str {
        match *self {
            GetBucketLoggingError::Validation(ref cause) => cause,
            GetBucketLoggingError::Credentials(ref err) => err.description(),
            GetBucketLoggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketLoggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketMetricsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketMetricsConfigurationError {
    pub fn from_body(body: &str) -> GetBucketMetricsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketMetricsConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketMetricsConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketMetricsConfigurationError {
    fn from(err: XmlParseError) -> GetBucketMetricsConfigurationError {
        let XmlParseError(message) = err;
        GetBucketMetricsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketMetricsConfigurationError {
    fn from(err: CredentialsError) -> GetBucketMetricsConfigurationError {
        GetBucketMetricsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketMetricsConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketMetricsConfigurationError {
        GetBucketMetricsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketMetricsConfigurationError {
    fn from(err: io::Error) -> GetBucketMetricsConfigurationError {
        GetBucketMetricsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketMetricsConfigurationError::Validation(ref cause) => cause,
            GetBucketMetricsConfigurationError::Credentials(ref err) => err.description(),
            GetBucketMetricsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketMetricsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketNotification
#[derive(Debug, PartialEq)]
pub enum GetBucketNotificationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketNotificationError {
    pub fn from_body(body: &str) -> GetBucketNotificationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketNotificationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketNotificationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketNotificationError {
    fn from(err: XmlParseError) -> GetBucketNotificationError {
        let XmlParseError(message) = err;
        GetBucketNotificationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketNotificationError {
    fn from(err: CredentialsError) -> GetBucketNotificationError {
        GetBucketNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketNotificationError {
    fn from(err: HttpDispatchError) -> GetBucketNotificationError {
        GetBucketNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketNotificationError {
    fn from(err: io::Error) -> GetBucketNotificationError {
        GetBucketNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketNotificationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketNotificationError::Validation(ref cause) => cause,
            GetBucketNotificationError::Credentials(ref err) => err.description(),
            GetBucketNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketNotificationConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketNotificationConfigurationError {
    pub fn from_body(body: &str) -> GetBucketNotificationConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketNotificationConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketNotificationConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketNotificationConfigurationError {
    fn from(err: XmlParseError) -> GetBucketNotificationConfigurationError {
        let XmlParseError(message) = err;
        GetBucketNotificationConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketNotificationConfigurationError {
    fn from(err: CredentialsError) -> GetBucketNotificationConfigurationError {
        GetBucketNotificationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketNotificationConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketNotificationConfigurationError {
        GetBucketNotificationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketNotificationConfigurationError {
    fn from(err: io::Error) -> GetBucketNotificationConfigurationError {
        GetBucketNotificationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketNotificationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketNotificationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketNotificationConfigurationError::Validation(ref cause) => cause,
            GetBucketNotificationConfigurationError::Credentials(ref err) => err.description(),
            GetBucketNotificationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketNotificationConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketPolicy
#[derive(Debug, PartialEq)]
pub enum GetBucketPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketPolicyError {
    pub fn from_body(body: &str) -> GetBucketPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketPolicyError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketPolicyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketPolicyError {
    fn from(err: XmlParseError) -> GetBucketPolicyError {
        let XmlParseError(message) = err;
        GetBucketPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketPolicyError {
    fn from(err: CredentialsError) -> GetBucketPolicyError {
        GetBucketPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketPolicyError {
    fn from(err: HttpDispatchError) -> GetBucketPolicyError {
        GetBucketPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketPolicyError {
    fn from(err: io::Error) -> GetBucketPolicyError {
        GetBucketPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetBucketPolicyError::Validation(ref cause) => cause,
            GetBucketPolicyError::Credentials(ref err) => err.description(),
            GetBucketPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketReplication
#[derive(Debug, PartialEq)]
pub enum GetBucketReplicationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketReplicationError {
    pub fn from_body(body: &str) -> GetBucketReplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketReplicationError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketReplicationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketReplicationError {
    fn from(err: XmlParseError) -> GetBucketReplicationError {
        let XmlParseError(message) = err;
        GetBucketReplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketReplicationError {
    fn from(err: CredentialsError) -> GetBucketReplicationError {
        GetBucketReplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketReplicationError {
    fn from(err: HttpDispatchError) -> GetBucketReplicationError {
        GetBucketReplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketReplicationError {
    fn from(err: io::Error) -> GetBucketReplicationError {
        GetBucketReplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketReplicationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketReplicationError::Validation(ref cause) => cause,
            GetBucketReplicationError::Credentials(ref err) => err.description(),
            GetBucketReplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketReplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketRequestPayment
#[derive(Debug, PartialEq)]
pub enum GetBucketRequestPaymentError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketRequestPaymentError {
    pub fn from_body(body: &str) -> GetBucketRequestPaymentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketRequestPaymentError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketRequestPaymentError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketRequestPaymentError {
    fn from(err: XmlParseError) -> GetBucketRequestPaymentError {
        let XmlParseError(message) = err;
        GetBucketRequestPaymentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketRequestPaymentError {
    fn from(err: CredentialsError) -> GetBucketRequestPaymentError {
        GetBucketRequestPaymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketRequestPaymentError {
    fn from(err: HttpDispatchError) -> GetBucketRequestPaymentError {
        GetBucketRequestPaymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketRequestPaymentError {
    fn from(err: io::Error) -> GetBucketRequestPaymentError {
        GetBucketRequestPaymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketRequestPaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketRequestPaymentError {
    fn description(&self) -> &str {
        match *self {
            GetBucketRequestPaymentError::Validation(ref cause) => cause,
            GetBucketRequestPaymentError::Credentials(ref err) => err.description(),
            GetBucketRequestPaymentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketRequestPaymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketTagging
#[derive(Debug, PartialEq)]
pub enum GetBucketTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketTaggingError {
    pub fn from_body(body: &str) -> GetBucketTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketTaggingError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketTaggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketTaggingError {
    fn from(err: XmlParseError) -> GetBucketTaggingError {
        let XmlParseError(message) = err;
        GetBucketTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketTaggingError {
    fn from(err: CredentialsError) -> GetBucketTaggingError {
        GetBucketTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketTaggingError {
    fn from(err: HttpDispatchError) -> GetBucketTaggingError {
        GetBucketTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketTaggingError {
    fn from(err: io::Error) -> GetBucketTaggingError {
        GetBucketTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketTaggingError {
    fn description(&self) -> &str {
        match *self {
            GetBucketTaggingError::Validation(ref cause) => cause,
            GetBucketTaggingError::Credentials(ref err) => err.description(),
            GetBucketTaggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketVersioning
#[derive(Debug, PartialEq)]
pub enum GetBucketVersioningError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketVersioningError {
    pub fn from_body(body: &str) -> GetBucketVersioningError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketVersioningError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketVersioningError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketVersioningError {
    fn from(err: XmlParseError) -> GetBucketVersioningError {
        let XmlParseError(message) = err;
        GetBucketVersioningError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketVersioningError {
    fn from(err: CredentialsError) -> GetBucketVersioningError {
        GetBucketVersioningError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketVersioningError {
    fn from(err: HttpDispatchError) -> GetBucketVersioningError {
        GetBucketVersioningError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketVersioningError {
    fn from(err: io::Error) -> GetBucketVersioningError {
        GetBucketVersioningError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketVersioningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketVersioningError {
    fn description(&self) -> &str {
        match *self {
            GetBucketVersioningError::Validation(ref cause) => cause,
            GetBucketVersioningError::Credentials(ref err) => err.description(),
            GetBucketVersioningError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketVersioningError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketWebsite
#[derive(Debug, PartialEq)]
pub enum GetBucketWebsiteError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBucketWebsiteError {
    pub fn from_body(body: &str) -> GetBucketWebsiteError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetBucketWebsiteError::Unknown(String::from(body)),
            },
            Err(_) => GetBucketWebsiteError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetBucketWebsiteError {
    fn from(err: XmlParseError) -> GetBucketWebsiteError {
        let XmlParseError(message) = err;
        GetBucketWebsiteError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketWebsiteError {
    fn from(err: CredentialsError) -> GetBucketWebsiteError {
        GetBucketWebsiteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketWebsiteError {
    fn from(err: HttpDispatchError) -> GetBucketWebsiteError {
        GetBucketWebsiteError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketWebsiteError {
    fn from(err: io::Error) -> GetBucketWebsiteError {
        GetBucketWebsiteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {
            GetBucketWebsiteError::Validation(ref cause) => cause,
            GetBucketWebsiteError::Credentials(ref err) => err.description(),
            GetBucketWebsiteError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketWebsiteError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObject
#[derive(Debug, PartialEq)]
pub enum GetObjectError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetObjectError {
    pub fn from_body(body: &str) -> GetObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "NoSuchKey" => GetObjectError::NoSuchKey(String::from(parsed_error.message)),
                _ => GetObjectError::Unknown(String::from(body)),
            },
            Err(_) => GetObjectError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetObjectError {
    fn from(err: XmlParseError) -> GetObjectError {
        let XmlParseError(message) = err;
        GetObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetObjectError {
    fn from(err: CredentialsError) -> GetObjectError {
        GetObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectError {
    fn from(err: HttpDispatchError) -> GetObjectError {
        GetObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectError {
    fn from(err: io::Error) -> GetObjectError {
        GetObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectError {
    fn description(&self) -> &str {
        match *self {
            GetObjectError::NoSuchKey(ref cause) => cause,
            GetObjectError::Validation(ref cause) => cause,
            GetObjectError::Credentials(ref err) => err.description(),
            GetObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObjectAcl
#[derive(Debug, PartialEq)]
pub enum GetObjectAclError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetObjectAclError {
    pub fn from_body(body: &str) -> GetObjectAclError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "NoSuchKey" => GetObjectAclError::NoSuchKey(String::from(parsed_error.message)),
                _ => GetObjectAclError::Unknown(String::from(body)),
            },
            Err(_) => GetObjectAclError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetObjectAclError {
    fn from(err: XmlParseError) -> GetObjectAclError {
        let XmlParseError(message) = err;
        GetObjectAclError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetObjectAclError {
    fn from(err: CredentialsError) -> GetObjectAclError {
        GetObjectAclError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectAclError {
    fn from(err: HttpDispatchError) -> GetObjectAclError {
        GetObjectAclError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectAclError {
    fn from(err: io::Error) -> GetObjectAclError {
        GetObjectAclError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectAclError {
    fn description(&self) -> &str {
        match *self {
            GetObjectAclError::NoSuchKey(ref cause) => cause,
            GetObjectAclError::Validation(ref cause) => cause,
            GetObjectAclError::Credentials(ref err) => err.description(),
            GetObjectAclError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectAclError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObjectTagging
#[derive(Debug, PartialEq)]
pub enum GetObjectTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetObjectTaggingError {
    pub fn from_body(body: &str) -> GetObjectTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetObjectTaggingError::Unknown(String::from(body)),
            },
            Err(_) => GetObjectTaggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetObjectTaggingError {
    fn from(err: XmlParseError) -> GetObjectTaggingError {
        let XmlParseError(message) = err;
        GetObjectTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetObjectTaggingError {
    fn from(err: CredentialsError) -> GetObjectTaggingError {
        GetObjectTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectTaggingError {
    fn from(err: HttpDispatchError) -> GetObjectTaggingError {
        GetObjectTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectTaggingError {
    fn from(err: io::Error) -> GetObjectTaggingError {
        GetObjectTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectTaggingError {
    fn description(&self) -> &str {
        match *self {
            GetObjectTaggingError::Validation(ref cause) => cause,
            GetObjectTaggingError::Credentials(ref err) => err.description(),
            GetObjectTaggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObjectTorrent
#[derive(Debug, PartialEq)]
pub enum GetObjectTorrentError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetObjectTorrentError {
    pub fn from_body(body: &str) -> GetObjectTorrentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetObjectTorrentError::Unknown(String::from(body)),
            },
            Err(_) => GetObjectTorrentError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetObjectTorrentError {
    fn from(err: XmlParseError) -> GetObjectTorrentError {
        let XmlParseError(message) = err;
        GetObjectTorrentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetObjectTorrentError {
    fn from(err: CredentialsError) -> GetObjectTorrentError {
        GetObjectTorrentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectTorrentError {
    fn from(err: HttpDispatchError) -> GetObjectTorrentError {
        GetObjectTorrentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectTorrentError {
    fn from(err: io::Error) -> GetObjectTorrentError {
        GetObjectTorrentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectTorrentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectTorrentError {
    fn description(&self) -> &str {
        match *self {
            GetObjectTorrentError::Validation(ref cause) => cause,
            GetObjectTorrentError::Credentials(ref err) => err.description(),
            GetObjectTorrentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectTorrentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by HeadBucket
#[derive(Debug, PartialEq)]
pub enum HeadBucketError {
    /// <p>The specified bucket does not exist.</p>
    NoSuchBucket(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl HeadBucketError {
    pub fn from_body(body: &str) -> HeadBucketError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "NoSuchBucket" => HeadBucketError::NoSuchBucket(String::from(parsed_error.message)),
                _ => HeadBucketError::Unknown(String::from(body)),
            },
            Err(_) => HeadBucketError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for HeadBucketError {
    fn from(err: XmlParseError) -> HeadBucketError {
        let XmlParseError(message) = err;
        HeadBucketError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for HeadBucketError {
    fn from(err: CredentialsError) -> HeadBucketError {
        HeadBucketError::Credentials(err)
    }
}
impl From<HttpDispatchError> for HeadBucketError {
    fn from(err: HttpDispatchError) -> HeadBucketError {
        HeadBucketError::HttpDispatch(err)
    }
}
impl From<io::Error> for HeadBucketError {
    fn from(err: io::Error) -> HeadBucketError {
        HeadBucketError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for HeadBucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for HeadBucketError {
    fn description(&self) -> &str {
        match *self {
            HeadBucketError::NoSuchBucket(ref cause) => cause,
            HeadBucketError::Validation(ref cause) => cause,
            HeadBucketError::Credentials(ref err) => err.description(),
            HeadBucketError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            HeadBucketError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by HeadObject
#[derive(Debug, PartialEq)]
pub enum HeadObjectError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl HeadObjectError {
    pub fn from_body(body: &str) -> HeadObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "NoSuchKey" => HeadObjectError::NoSuchKey(String::from(parsed_error.message)),
                _ => HeadObjectError::Unknown(String::from(body)),
            },
            Err(_) => HeadObjectError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for HeadObjectError {
    fn from(err: XmlParseError) -> HeadObjectError {
        let XmlParseError(message) = err;
        HeadObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for HeadObjectError {
    fn from(err: CredentialsError) -> HeadObjectError {
        HeadObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for HeadObjectError {
    fn from(err: HttpDispatchError) -> HeadObjectError {
        HeadObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for HeadObjectError {
    fn from(err: io::Error) -> HeadObjectError {
        HeadObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for HeadObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for HeadObjectError {
    fn description(&self) -> &str {
        match *self {
            HeadObjectError::NoSuchKey(ref cause) => cause,
            HeadObjectError::Validation(ref cause) => cause,
            HeadObjectError::Credentials(ref err) => err.description(),
            HeadObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            HeadObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBucketAnalyticsConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketAnalyticsConfigurationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBucketAnalyticsConfigurationsError {
    pub fn from_body(body: &str) -> ListBucketAnalyticsConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListBucketAnalyticsConfigurationsError::Unknown(String::from(body)),
            },
            Err(_) => ListBucketAnalyticsConfigurationsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListBucketAnalyticsConfigurationsError {
    fn from(err: XmlParseError) -> ListBucketAnalyticsConfigurationsError {
        let XmlParseError(message) = err;
        ListBucketAnalyticsConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListBucketAnalyticsConfigurationsError {
    fn from(err: CredentialsError) -> ListBucketAnalyticsConfigurationsError {
        ListBucketAnalyticsConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBucketAnalyticsConfigurationsError {
    fn from(err: HttpDispatchError) -> ListBucketAnalyticsConfigurationsError {
        ListBucketAnalyticsConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBucketAnalyticsConfigurationsError {
    fn from(err: io::Error) -> ListBucketAnalyticsConfigurationsError {
        ListBucketAnalyticsConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBucketAnalyticsConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketAnalyticsConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListBucketAnalyticsConfigurationsError::Validation(ref cause) => cause,
            ListBucketAnalyticsConfigurationsError::Credentials(ref err) => err.description(),
            ListBucketAnalyticsConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBucketAnalyticsConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBucketInventoryConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketInventoryConfigurationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBucketInventoryConfigurationsError {
    pub fn from_body(body: &str) -> ListBucketInventoryConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListBucketInventoryConfigurationsError::Unknown(String::from(body)),
            },
            Err(_) => ListBucketInventoryConfigurationsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListBucketInventoryConfigurationsError {
    fn from(err: XmlParseError) -> ListBucketInventoryConfigurationsError {
        let XmlParseError(message) = err;
        ListBucketInventoryConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListBucketInventoryConfigurationsError {
    fn from(err: CredentialsError) -> ListBucketInventoryConfigurationsError {
        ListBucketInventoryConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBucketInventoryConfigurationsError {
    fn from(err: HttpDispatchError) -> ListBucketInventoryConfigurationsError {
        ListBucketInventoryConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBucketInventoryConfigurationsError {
    fn from(err: io::Error) -> ListBucketInventoryConfigurationsError {
        ListBucketInventoryConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBucketInventoryConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketInventoryConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListBucketInventoryConfigurationsError::Validation(ref cause) => cause,
            ListBucketInventoryConfigurationsError::Credentials(ref err) => err.description(),
            ListBucketInventoryConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBucketInventoryConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBucketMetricsConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketMetricsConfigurationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBucketMetricsConfigurationsError {
    pub fn from_body(body: &str) -> ListBucketMetricsConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListBucketMetricsConfigurationsError::Unknown(String::from(body)),
            },
            Err(_) => ListBucketMetricsConfigurationsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListBucketMetricsConfigurationsError {
    fn from(err: XmlParseError) -> ListBucketMetricsConfigurationsError {
        let XmlParseError(message) = err;
        ListBucketMetricsConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListBucketMetricsConfigurationsError {
    fn from(err: CredentialsError) -> ListBucketMetricsConfigurationsError {
        ListBucketMetricsConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBucketMetricsConfigurationsError {
    fn from(err: HttpDispatchError) -> ListBucketMetricsConfigurationsError {
        ListBucketMetricsConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBucketMetricsConfigurationsError {
    fn from(err: io::Error) -> ListBucketMetricsConfigurationsError {
        ListBucketMetricsConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBucketMetricsConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketMetricsConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListBucketMetricsConfigurationsError::Validation(ref cause) => cause,
            ListBucketMetricsConfigurationsError::Credentials(ref err) => err.description(),
            ListBucketMetricsConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBucketMetricsConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBuckets
#[derive(Debug, PartialEq)]
pub enum ListBucketsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBucketsError {
    pub fn from_body(body: &str) -> ListBucketsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListBucketsError::Unknown(String::from(body)),
            },
            Err(_) => ListBucketsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListBucketsError {
    fn from(err: XmlParseError) -> ListBucketsError {
        let XmlParseError(message) = err;
        ListBucketsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListBucketsError {
    fn from(err: CredentialsError) -> ListBucketsError {
        ListBucketsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBucketsError {
    fn from(err: HttpDispatchError) -> ListBucketsError {
        ListBucketsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBucketsError {
    fn from(err: io::Error) -> ListBucketsError {
        ListBucketsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBucketsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketsError {
    fn description(&self) -> &str {
        match *self {
            ListBucketsError::Validation(ref cause) => cause,
            ListBucketsError::Credentials(ref err) => err.description(),
            ListBucketsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBucketsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListMultipartUploads
#[derive(Debug, PartialEq)]
pub enum ListMultipartUploadsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListMultipartUploadsError {
    pub fn from_body(body: &str) -> ListMultipartUploadsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListMultipartUploadsError::Unknown(String::from(body)),
            },
            Err(_) => ListMultipartUploadsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListMultipartUploadsError {
    fn from(err: XmlParseError) -> ListMultipartUploadsError {
        let XmlParseError(message) = err;
        ListMultipartUploadsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListMultipartUploadsError {
    fn from(err: CredentialsError) -> ListMultipartUploadsError {
        ListMultipartUploadsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListMultipartUploadsError {
    fn from(err: HttpDispatchError) -> ListMultipartUploadsError {
        ListMultipartUploadsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListMultipartUploadsError {
    fn from(err: io::Error) -> ListMultipartUploadsError {
        ListMultipartUploadsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListMultipartUploadsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMultipartUploadsError {
    fn description(&self) -> &str {
        match *self {
            ListMultipartUploadsError::Validation(ref cause) => cause,
            ListMultipartUploadsError::Credentials(ref err) => err.description(),
            ListMultipartUploadsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListMultipartUploadsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectVersions
#[derive(Debug, PartialEq)]
pub enum ListObjectVersionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListObjectVersionsError {
    pub fn from_body(body: &str) -> ListObjectVersionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListObjectVersionsError::Unknown(String::from(body)),
            },
            Err(_) => ListObjectVersionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListObjectVersionsError {
    fn from(err: XmlParseError) -> ListObjectVersionsError {
        let XmlParseError(message) = err;
        ListObjectVersionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListObjectVersionsError {
    fn from(err: CredentialsError) -> ListObjectVersionsError {
        ListObjectVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectVersionsError {
    fn from(err: HttpDispatchError) -> ListObjectVersionsError {
        ListObjectVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectVersionsError {
    fn from(err: io::Error) -> ListObjectVersionsError {
        ListObjectVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListObjectVersionsError::Validation(ref cause) => cause,
            ListObjectVersionsError::Credentials(ref err) => err.description(),
            ListObjectVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjects
#[derive(Debug, PartialEq)]
pub enum ListObjectsError {
    /// <p>The specified bucket does not exist.</p>
    NoSuchBucket(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListObjectsError {
    pub fn from_body(body: &str) -> ListObjectsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "NoSuchBucket" => {
                    ListObjectsError::NoSuchBucket(String::from(parsed_error.message))
                }
                _ => ListObjectsError::Unknown(String::from(body)),
            },
            Err(_) => ListObjectsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListObjectsError {
    fn from(err: XmlParseError) -> ListObjectsError {
        let XmlParseError(message) = err;
        ListObjectsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListObjectsError {
    fn from(err: CredentialsError) -> ListObjectsError {
        ListObjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectsError {
    fn from(err: HttpDispatchError) -> ListObjectsError {
        ListObjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectsError {
    fn from(err: io::Error) -> ListObjectsError {
        ListObjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectsError {
    fn description(&self) -> &str {
        match *self {
            ListObjectsError::NoSuchBucket(ref cause) => cause,
            ListObjectsError::Validation(ref cause) => cause,
            ListObjectsError::Credentials(ref err) => err.description(),
            ListObjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListObjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectsV2
#[derive(Debug, PartialEq)]
pub enum ListObjectsV2Error {
    /// <p>The specified bucket does not exist.</p>
    NoSuchBucket(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListObjectsV2Error {
    pub fn from_body(body: &str) -> ListObjectsV2Error {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "NoSuchBucket" => {
                    ListObjectsV2Error::NoSuchBucket(String::from(parsed_error.message))
                }
                _ => ListObjectsV2Error::Unknown(String::from(body)),
            },
            Err(_) => ListObjectsV2Error::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListObjectsV2Error {
    fn from(err: XmlParseError) -> ListObjectsV2Error {
        let XmlParseError(message) = err;
        ListObjectsV2Error::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListObjectsV2Error {
    fn from(err: CredentialsError) -> ListObjectsV2Error {
        ListObjectsV2Error::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectsV2Error {
    fn from(err: HttpDispatchError) -> ListObjectsV2Error {
        ListObjectsV2Error::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectsV2Error {
    fn from(err: io::Error) -> ListObjectsV2Error {
        ListObjectsV2Error::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectsV2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectsV2Error {
    fn description(&self) -> &str {
        match *self {
            ListObjectsV2Error::NoSuchBucket(ref cause) => cause,
            ListObjectsV2Error::Validation(ref cause) => cause,
            ListObjectsV2Error::Credentials(ref err) => err.description(),
            ListObjectsV2Error::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListObjectsV2Error::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListParts
#[derive(Debug, PartialEq)]
pub enum ListPartsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPartsError {
    pub fn from_body(body: &str) -> ListPartsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListPartsError::Unknown(String::from(body)),
            },
            Err(_) => ListPartsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListPartsError {
    fn from(err: XmlParseError) -> ListPartsError {
        let XmlParseError(message) = err;
        ListPartsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListPartsError {
    fn from(err: CredentialsError) -> ListPartsError {
        ListPartsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPartsError {
    fn from(err: HttpDispatchError) -> ListPartsError {
        ListPartsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPartsError {
    fn from(err: io::Error) -> ListPartsError {
        ListPartsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPartsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPartsError {
    fn description(&self) -> &str {
        match *self {
            ListPartsError::Validation(ref cause) => cause,
            ListPartsError::Credentials(ref err) => err.description(),
            ListPartsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPartsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketAccelerateConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketAccelerateConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketAccelerateConfigurationError {
    pub fn from_body(body: &str) -> PutBucketAccelerateConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketAccelerateConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketAccelerateConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketAccelerateConfigurationError {
    fn from(err: XmlParseError) -> PutBucketAccelerateConfigurationError {
        let XmlParseError(message) = err;
        PutBucketAccelerateConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketAccelerateConfigurationError {
    fn from(err: CredentialsError) -> PutBucketAccelerateConfigurationError {
        PutBucketAccelerateConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketAccelerateConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketAccelerateConfigurationError {
        PutBucketAccelerateConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketAccelerateConfigurationError {
    fn from(err: io::Error) -> PutBucketAccelerateConfigurationError {
        PutBucketAccelerateConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketAccelerateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAccelerateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketAccelerateConfigurationError::Validation(ref cause) => cause,
            PutBucketAccelerateConfigurationError::Credentials(ref err) => err.description(),
            PutBucketAccelerateConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketAccelerateConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketAcl
#[derive(Debug, PartialEq)]
pub enum PutBucketAclError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketAclError {
    pub fn from_body(body: &str) -> PutBucketAclError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketAclError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketAclError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketAclError {
    fn from(err: XmlParseError) -> PutBucketAclError {
        let XmlParseError(message) = err;
        PutBucketAclError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketAclError {
    fn from(err: CredentialsError) -> PutBucketAclError {
        PutBucketAclError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketAclError {
    fn from(err: HttpDispatchError) -> PutBucketAclError {
        PutBucketAclError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketAclError {
    fn from(err: io::Error) -> PutBucketAclError {
        PutBucketAclError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAclError {
    fn description(&self) -> &str {
        match *self {
            PutBucketAclError::Validation(ref cause) => cause,
            PutBucketAclError::Credentials(ref err) => err.description(),
            PutBucketAclError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketAclError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketAnalyticsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketAnalyticsConfigurationError {
    pub fn from_body(body: &str) -> PutBucketAnalyticsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketAnalyticsConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketAnalyticsConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketAnalyticsConfigurationError {
    fn from(err: XmlParseError) -> PutBucketAnalyticsConfigurationError {
        let XmlParseError(message) = err;
        PutBucketAnalyticsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketAnalyticsConfigurationError {
    fn from(err: CredentialsError) -> PutBucketAnalyticsConfigurationError {
        PutBucketAnalyticsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketAnalyticsConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketAnalyticsConfigurationError {
        PutBucketAnalyticsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketAnalyticsConfigurationError {
    fn from(err: io::Error) -> PutBucketAnalyticsConfigurationError {
        PutBucketAnalyticsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketAnalyticsConfigurationError::Validation(ref cause) => cause,
            PutBucketAnalyticsConfigurationError::Credentials(ref err) => err.description(),
            PutBucketAnalyticsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketAnalyticsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketCors
#[derive(Debug, PartialEq)]
pub enum PutBucketCorsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketCorsError {
    pub fn from_body(body: &str) -> PutBucketCorsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketCorsError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketCorsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketCorsError {
    fn from(err: XmlParseError) -> PutBucketCorsError {
        let XmlParseError(message) = err;
        PutBucketCorsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketCorsError {
    fn from(err: CredentialsError) -> PutBucketCorsError {
        PutBucketCorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketCorsError {
    fn from(err: HttpDispatchError) -> PutBucketCorsError {
        PutBucketCorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketCorsError {
    fn from(err: io::Error) -> PutBucketCorsError {
        PutBucketCorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketCorsError {
    fn description(&self) -> &str {
        match *self {
            PutBucketCorsError::Validation(ref cause) => cause,
            PutBucketCorsError::Credentials(ref err) => err.description(),
            PutBucketCorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketCorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketEncryption
#[derive(Debug, PartialEq)]
pub enum PutBucketEncryptionError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketEncryptionError {
    pub fn from_body(body: &str) -> PutBucketEncryptionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketEncryptionError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketEncryptionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketEncryptionError {
    fn from(err: XmlParseError) -> PutBucketEncryptionError {
        let XmlParseError(message) = err;
        PutBucketEncryptionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketEncryptionError {
    fn from(err: CredentialsError) -> PutBucketEncryptionError {
        PutBucketEncryptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketEncryptionError {
    fn from(err: HttpDispatchError) -> PutBucketEncryptionError {
        PutBucketEncryptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketEncryptionError {
    fn from(err: io::Error) -> PutBucketEncryptionError {
        PutBucketEncryptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketEncryptionError {
    fn description(&self) -> &str {
        match *self {
            PutBucketEncryptionError::Validation(ref cause) => cause,
            PutBucketEncryptionError::Credentials(ref err) => err.description(),
            PutBucketEncryptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketEncryptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketInventoryConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketInventoryConfigurationError {
    pub fn from_body(body: &str) -> PutBucketInventoryConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketInventoryConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketInventoryConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketInventoryConfigurationError {
    fn from(err: XmlParseError) -> PutBucketInventoryConfigurationError {
        let XmlParseError(message) = err;
        PutBucketInventoryConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketInventoryConfigurationError {
    fn from(err: CredentialsError) -> PutBucketInventoryConfigurationError {
        PutBucketInventoryConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketInventoryConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketInventoryConfigurationError {
        PutBucketInventoryConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketInventoryConfigurationError {
    fn from(err: io::Error) -> PutBucketInventoryConfigurationError {
        PutBucketInventoryConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketInventoryConfigurationError::Validation(ref cause) => cause,
            PutBucketInventoryConfigurationError::Credentials(ref err) => err.description(),
            PutBucketInventoryConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketInventoryConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum PutBucketLifecycleError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketLifecycleError {
    pub fn from_body(body: &str) -> PutBucketLifecycleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketLifecycleError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketLifecycleError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketLifecycleError {
    fn from(err: XmlParseError) -> PutBucketLifecycleError {
        let XmlParseError(message) = err;
        PutBucketLifecycleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketLifecycleError {
    fn from(err: CredentialsError) -> PutBucketLifecycleError {
        PutBucketLifecycleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketLifecycleError {
    fn from(err: HttpDispatchError) -> PutBucketLifecycleError {
        PutBucketLifecycleError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketLifecycleError {
    fn from(err: io::Error) -> PutBucketLifecycleError {
        PutBucketLifecycleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {
            PutBucketLifecycleError::Validation(ref cause) => cause,
            PutBucketLifecycleError::Credentials(ref err) => err.description(),
            PutBucketLifecycleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketLifecycleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketLifecycleConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketLifecycleConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketLifecycleConfigurationError {
    pub fn from_body(body: &str) -> PutBucketLifecycleConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketLifecycleConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketLifecycleConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketLifecycleConfigurationError {
    fn from(err: XmlParseError) -> PutBucketLifecycleConfigurationError {
        let XmlParseError(message) = err;
        PutBucketLifecycleConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketLifecycleConfigurationError {
    fn from(err: CredentialsError) -> PutBucketLifecycleConfigurationError {
        PutBucketLifecycleConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketLifecycleConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketLifecycleConfigurationError {
        PutBucketLifecycleConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketLifecycleConfigurationError {
    fn from(err: io::Error) -> PutBucketLifecycleConfigurationError {
        PutBucketLifecycleConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketLifecycleConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLifecycleConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketLifecycleConfigurationError::Validation(ref cause) => cause,
            PutBucketLifecycleConfigurationError::Credentials(ref err) => err.description(),
            PutBucketLifecycleConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketLifecycleConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketLogging
#[derive(Debug, PartialEq)]
pub enum PutBucketLoggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketLoggingError {
    pub fn from_body(body: &str) -> PutBucketLoggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketLoggingError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketLoggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketLoggingError {
    fn from(err: XmlParseError) -> PutBucketLoggingError {
        let XmlParseError(message) = err;
        PutBucketLoggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketLoggingError {
    fn from(err: CredentialsError) -> PutBucketLoggingError {
        PutBucketLoggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketLoggingError {
    fn from(err: HttpDispatchError) -> PutBucketLoggingError {
        PutBucketLoggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketLoggingError {
    fn from(err: io::Error) -> PutBucketLoggingError {
        PutBucketLoggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLoggingError {
    fn description(&self) -> &str {
        match *self {
            PutBucketLoggingError::Validation(ref cause) => cause,
            PutBucketLoggingError::Credentials(ref err) => err.description(),
            PutBucketLoggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketLoggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketMetricsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketMetricsConfigurationError {
    pub fn from_body(body: &str) -> PutBucketMetricsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketMetricsConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketMetricsConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketMetricsConfigurationError {
    fn from(err: XmlParseError) -> PutBucketMetricsConfigurationError {
        let XmlParseError(message) = err;
        PutBucketMetricsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketMetricsConfigurationError {
    fn from(err: CredentialsError) -> PutBucketMetricsConfigurationError {
        PutBucketMetricsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketMetricsConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketMetricsConfigurationError {
        PutBucketMetricsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketMetricsConfigurationError {
    fn from(err: io::Error) -> PutBucketMetricsConfigurationError {
        PutBucketMetricsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketMetricsConfigurationError::Validation(ref cause) => cause,
            PutBucketMetricsConfigurationError::Credentials(ref err) => err.description(),
            PutBucketMetricsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketMetricsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketNotification
#[derive(Debug, PartialEq)]
pub enum PutBucketNotificationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketNotificationError {
    pub fn from_body(body: &str) -> PutBucketNotificationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketNotificationError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketNotificationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketNotificationError {
    fn from(err: XmlParseError) -> PutBucketNotificationError {
        let XmlParseError(message) = err;
        PutBucketNotificationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketNotificationError {
    fn from(err: CredentialsError) -> PutBucketNotificationError {
        PutBucketNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketNotificationError {
    fn from(err: HttpDispatchError) -> PutBucketNotificationError {
        PutBucketNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketNotificationError {
    fn from(err: io::Error) -> PutBucketNotificationError {
        PutBucketNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketNotificationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketNotificationError::Validation(ref cause) => cause,
            PutBucketNotificationError::Credentials(ref err) => err.description(),
            PutBucketNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketNotificationConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketNotificationConfigurationError {
    pub fn from_body(body: &str) -> PutBucketNotificationConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketNotificationConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketNotificationConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketNotificationConfigurationError {
    fn from(err: XmlParseError) -> PutBucketNotificationConfigurationError {
        let XmlParseError(message) = err;
        PutBucketNotificationConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketNotificationConfigurationError {
    fn from(err: CredentialsError) -> PutBucketNotificationConfigurationError {
        PutBucketNotificationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketNotificationConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketNotificationConfigurationError {
        PutBucketNotificationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketNotificationConfigurationError {
    fn from(err: io::Error) -> PutBucketNotificationConfigurationError {
        PutBucketNotificationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketNotificationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketNotificationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketNotificationConfigurationError::Validation(ref cause) => cause,
            PutBucketNotificationConfigurationError::Credentials(ref err) => err.description(),
            PutBucketNotificationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketNotificationConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketPolicy
#[derive(Debug, PartialEq)]
pub enum PutBucketPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketPolicyError {
    pub fn from_body(body: &str) -> PutBucketPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketPolicyError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketPolicyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketPolicyError {
    fn from(err: XmlParseError) -> PutBucketPolicyError {
        let XmlParseError(message) = err;
        PutBucketPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketPolicyError {
    fn from(err: CredentialsError) -> PutBucketPolicyError {
        PutBucketPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketPolicyError {
    fn from(err: HttpDispatchError) -> PutBucketPolicyError {
        PutBucketPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketPolicyError {
    fn from(err: io::Error) -> PutBucketPolicyError {
        PutBucketPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutBucketPolicyError::Validation(ref cause) => cause,
            PutBucketPolicyError::Credentials(ref err) => err.description(),
            PutBucketPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketReplication
#[derive(Debug, PartialEq)]
pub enum PutBucketReplicationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketReplicationError {
    pub fn from_body(body: &str) -> PutBucketReplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketReplicationError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketReplicationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketReplicationError {
    fn from(err: XmlParseError) -> PutBucketReplicationError {
        let XmlParseError(message) = err;
        PutBucketReplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketReplicationError {
    fn from(err: CredentialsError) -> PutBucketReplicationError {
        PutBucketReplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketReplicationError {
    fn from(err: HttpDispatchError) -> PutBucketReplicationError {
        PutBucketReplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketReplicationError {
    fn from(err: io::Error) -> PutBucketReplicationError {
        PutBucketReplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketReplicationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketReplicationError::Validation(ref cause) => cause,
            PutBucketReplicationError::Credentials(ref err) => err.description(),
            PutBucketReplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketReplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketRequestPayment
#[derive(Debug, PartialEq)]
pub enum PutBucketRequestPaymentError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketRequestPaymentError {
    pub fn from_body(body: &str) -> PutBucketRequestPaymentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketRequestPaymentError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketRequestPaymentError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketRequestPaymentError {
    fn from(err: XmlParseError) -> PutBucketRequestPaymentError {
        let XmlParseError(message) = err;
        PutBucketRequestPaymentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketRequestPaymentError {
    fn from(err: CredentialsError) -> PutBucketRequestPaymentError {
        PutBucketRequestPaymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketRequestPaymentError {
    fn from(err: HttpDispatchError) -> PutBucketRequestPaymentError {
        PutBucketRequestPaymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketRequestPaymentError {
    fn from(err: io::Error) -> PutBucketRequestPaymentError {
        PutBucketRequestPaymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketRequestPaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketRequestPaymentError {
    fn description(&self) -> &str {
        match *self {
            PutBucketRequestPaymentError::Validation(ref cause) => cause,
            PutBucketRequestPaymentError::Credentials(ref err) => err.description(),
            PutBucketRequestPaymentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketRequestPaymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketTagging
#[derive(Debug, PartialEq)]
pub enum PutBucketTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketTaggingError {
    pub fn from_body(body: &str) -> PutBucketTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketTaggingError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketTaggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketTaggingError {
    fn from(err: XmlParseError) -> PutBucketTaggingError {
        let XmlParseError(message) = err;
        PutBucketTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketTaggingError {
    fn from(err: CredentialsError) -> PutBucketTaggingError {
        PutBucketTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketTaggingError {
    fn from(err: HttpDispatchError) -> PutBucketTaggingError {
        PutBucketTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketTaggingError {
    fn from(err: io::Error) -> PutBucketTaggingError {
        PutBucketTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketTaggingError {
    fn description(&self) -> &str {
        match *self {
            PutBucketTaggingError::Validation(ref cause) => cause,
            PutBucketTaggingError::Credentials(ref err) => err.description(),
            PutBucketTaggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketVersioning
#[derive(Debug, PartialEq)]
pub enum PutBucketVersioningError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketVersioningError {
    pub fn from_body(body: &str) -> PutBucketVersioningError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketVersioningError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketVersioningError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketVersioningError {
    fn from(err: XmlParseError) -> PutBucketVersioningError {
        let XmlParseError(message) = err;
        PutBucketVersioningError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketVersioningError {
    fn from(err: CredentialsError) -> PutBucketVersioningError {
        PutBucketVersioningError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketVersioningError {
    fn from(err: HttpDispatchError) -> PutBucketVersioningError {
        PutBucketVersioningError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketVersioningError {
    fn from(err: io::Error) -> PutBucketVersioningError {
        PutBucketVersioningError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketVersioningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketVersioningError {
    fn description(&self) -> &str {
        match *self {
            PutBucketVersioningError::Validation(ref cause) => cause,
            PutBucketVersioningError::Credentials(ref err) => err.description(),
            PutBucketVersioningError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketVersioningError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketWebsite
#[derive(Debug, PartialEq)]
pub enum PutBucketWebsiteError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutBucketWebsiteError {
    pub fn from_body(body: &str) -> PutBucketWebsiteError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutBucketWebsiteError::Unknown(String::from(body)),
            },
            Err(_) => PutBucketWebsiteError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutBucketWebsiteError {
    fn from(err: XmlParseError) -> PutBucketWebsiteError {
        let XmlParseError(message) = err;
        PutBucketWebsiteError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketWebsiteError {
    fn from(err: CredentialsError) -> PutBucketWebsiteError {
        PutBucketWebsiteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketWebsiteError {
    fn from(err: HttpDispatchError) -> PutBucketWebsiteError {
        PutBucketWebsiteError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketWebsiteError {
    fn from(err: io::Error) -> PutBucketWebsiteError {
        PutBucketWebsiteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {
            PutBucketWebsiteError::Validation(ref cause) => cause,
            PutBucketWebsiteError::Credentials(ref err) => err.description(),
            PutBucketWebsiteError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketWebsiteError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutObject
#[derive(Debug, PartialEq)]
pub enum PutObjectError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutObjectError {
    pub fn from_body(body: &str) -> PutObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutObjectError::Unknown(String::from(body)),
            },
            Err(_) => PutObjectError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutObjectError {
    fn from(err: XmlParseError) -> PutObjectError {
        let XmlParseError(message) = err;
        PutObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutObjectError {
    fn from(err: CredentialsError) -> PutObjectError {
        PutObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutObjectError {
    fn from(err: HttpDispatchError) -> PutObjectError {
        PutObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutObjectError {
    fn from(err: io::Error) -> PutObjectError {
        PutObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectError {
    fn description(&self) -> &str {
        match *self {
            PutObjectError::Validation(ref cause) => cause,
            PutObjectError::Credentials(ref err) => err.description(),
            PutObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutObjectAcl
#[derive(Debug, PartialEq)]
pub enum PutObjectAclError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutObjectAclError {
    pub fn from_body(body: &str) -> PutObjectAclError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "NoSuchKey" => PutObjectAclError::NoSuchKey(String::from(parsed_error.message)),
                _ => PutObjectAclError::Unknown(String::from(body)),
            },
            Err(_) => PutObjectAclError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutObjectAclError {
    fn from(err: XmlParseError) -> PutObjectAclError {
        let XmlParseError(message) = err;
        PutObjectAclError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutObjectAclError {
    fn from(err: CredentialsError) -> PutObjectAclError {
        PutObjectAclError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutObjectAclError {
    fn from(err: HttpDispatchError) -> PutObjectAclError {
        PutObjectAclError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutObjectAclError {
    fn from(err: io::Error) -> PutObjectAclError {
        PutObjectAclError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutObjectAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectAclError {
    fn description(&self) -> &str {
        match *self {
            PutObjectAclError::NoSuchKey(ref cause) => cause,
            PutObjectAclError::Validation(ref cause) => cause,
            PutObjectAclError::Credentials(ref err) => err.description(),
            PutObjectAclError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutObjectAclError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutObjectTagging
#[derive(Debug, PartialEq)]
pub enum PutObjectTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutObjectTaggingError {
    pub fn from_body(body: &str) -> PutObjectTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => PutObjectTaggingError::Unknown(String::from(body)),
            },
            Err(_) => PutObjectTaggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutObjectTaggingError {
    fn from(err: XmlParseError) -> PutObjectTaggingError {
        let XmlParseError(message) = err;
        PutObjectTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutObjectTaggingError {
    fn from(err: CredentialsError) -> PutObjectTaggingError {
        PutObjectTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutObjectTaggingError {
    fn from(err: HttpDispatchError) -> PutObjectTaggingError {
        PutObjectTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutObjectTaggingError {
    fn from(err: io::Error) -> PutObjectTaggingError {
        PutObjectTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectTaggingError {
    fn description(&self) -> &str {
        match *self {
            PutObjectTaggingError::Validation(ref cause) => cause,
            PutObjectTaggingError::Credentials(ref err) => err.description(),
            PutObjectTaggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutObjectTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreObject
#[derive(Debug, PartialEq)]
pub enum RestoreObjectError {
    /// <p>This operation is not allowed against this storage tier</p>
    ObjectAlreadyInActiveTierError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RestoreObjectError {
    pub fn from_body(body: &str) -> RestoreObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ObjectAlreadyInActiveTierError" => {
                    RestoreObjectError::ObjectAlreadyInActiveTierError(String::from(
                        parsed_error.message,
                    ))
                }
                _ => RestoreObjectError::Unknown(String::from(body)),
            },
            Err(_) => RestoreObjectError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RestoreObjectError {
    fn from(err: XmlParseError) -> RestoreObjectError {
        let XmlParseError(message) = err;
        RestoreObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RestoreObjectError {
    fn from(err: CredentialsError) -> RestoreObjectError {
        RestoreObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreObjectError {
    fn from(err: HttpDispatchError) -> RestoreObjectError {
        RestoreObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreObjectError {
    fn from(err: io::Error) -> RestoreObjectError {
        RestoreObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreObjectError {
    fn description(&self) -> &str {
        match *self {
            RestoreObjectError::ObjectAlreadyInActiveTierError(ref cause) => cause,
            RestoreObjectError::Validation(ref cause) => cause,
            RestoreObjectError::Credentials(ref err) => err.description(),
            RestoreObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RestoreObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SelectObjectContent
#[derive(Debug, PartialEq)]
pub enum SelectObjectContentError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SelectObjectContentError {
    pub fn from_body(body: &str) -> SelectObjectContentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SelectObjectContentError::Unknown(String::from(body)),
            },
            Err(_) => SelectObjectContentError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SelectObjectContentError {
    fn from(err: XmlParseError) -> SelectObjectContentError {
        let XmlParseError(message) = err;
        SelectObjectContentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SelectObjectContentError {
    fn from(err: CredentialsError) -> SelectObjectContentError {
        SelectObjectContentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SelectObjectContentError {
    fn from(err: HttpDispatchError) -> SelectObjectContentError {
        SelectObjectContentError::HttpDispatch(err)
    }
}
impl From<io::Error> for SelectObjectContentError {
    fn from(err: io::Error) -> SelectObjectContentError {
        SelectObjectContentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SelectObjectContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SelectObjectContentError {
    fn description(&self) -> &str {
        match *self {
            SelectObjectContentError::Validation(ref cause) => cause,
            SelectObjectContentError::Credentials(ref err) => err.description(),
            SelectObjectContentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SelectObjectContentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UploadPart
#[derive(Debug, PartialEq)]
pub enum UploadPartError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UploadPartError {
    pub fn from_body(body: &str) -> UploadPartError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => UploadPartError::Unknown(String::from(body)),
            },
            Err(_) => UploadPartError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UploadPartError {
    fn from(err: XmlParseError) -> UploadPartError {
        let XmlParseError(message) = err;
        UploadPartError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UploadPartError {
    fn from(err: CredentialsError) -> UploadPartError {
        UploadPartError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UploadPartError {
    fn from(err: HttpDispatchError) -> UploadPartError {
        UploadPartError::HttpDispatch(err)
    }
}
impl From<io::Error> for UploadPartError {
    fn from(err: io::Error) -> UploadPartError {
        UploadPartError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UploadPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadPartError {
    fn description(&self) -> &str {
        match *self {
            UploadPartError::Validation(ref cause) => cause,
            UploadPartError::Credentials(ref err) => err.description(),
            UploadPartError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UploadPartError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UploadPartCopy
#[derive(Debug, PartialEq)]
pub enum UploadPartCopyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UploadPartCopyError {
    pub fn from_body(body: &str) -> UploadPartCopyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => UploadPartCopyError::Unknown(String::from(body)),
            },
            Err(_) => UploadPartCopyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UploadPartCopyError {
    fn from(err: XmlParseError) -> UploadPartCopyError {
        let XmlParseError(message) = err;
        UploadPartCopyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UploadPartCopyError {
    fn from(err: CredentialsError) -> UploadPartCopyError {
        UploadPartCopyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UploadPartCopyError {
    fn from(err: HttpDispatchError) -> UploadPartCopyError {
        UploadPartCopyError::HttpDispatch(err)
    }
}
impl From<io::Error> for UploadPartCopyError {
    fn from(err: io::Error) -> UploadPartCopyError {
        UploadPartCopyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UploadPartCopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadPartCopyError {
    fn description(&self) -> &str {
        match *self {
            UploadPartCopyError::Validation(ref cause) => cause,
            UploadPartCopyError::Credentials(ref err) => err.description(),
            UploadPartCopyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UploadPartCopyError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon S3 API. Amazon S3 clients implement this trait.
pub trait S3 {
    /// <p>Aborts a multipart upload.</p> <p>To verify that all parts have been removed, so you don't get charged for the part storage, you should call the List Parts operation and ensure the parts list is empty.</p>
    fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadRequest,
    ) -> RusotoFuture<AbortMultipartUploadOutput, AbortMultipartUploadError>;

    /// <p>Completes a multipart upload by assembling previously uploaded parts.</p>
    fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadRequest,
    ) -> RusotoFuture<CompleteMultipartUploadOutput, CompleteMultipartUploadError>;

    /// <p>Creates a copy of an object that is already stored in Amazon S3.</p>
    fn copy_object(
        &self,
        input: CopyObjectRequest,
    ) -> RusotoFuture<CopyObjectOutput, CopyObjectError>;

    /// <p>Creates a new bucket.</p>
    fn create_bucket(
        &self,
        input: CreateBucketRequest,
    ) -> RusotoFuture<CreateBucketOutput, CreateBucketError>;

    /// <p>Initiates a multipart upload and returns an upload ID.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    fn create_multipart_upload(
        &self,
        input: CreateMultipartUploadRequest,
    ) -> RusotoFuture<CreateMultipartUploadOutput, CreateMultipartUploadError>;

    /// <p>Deletes the bucket. All objects (including all object versions and Delete Markers) in the bucket must be deleted before the bucket itself can be deleted.</p>
    fn delete_bucket(&self, input: DeleteBucketRequest) -> RusotoFuture<(), DeleteBucketError>;

    /// <p>Deletes an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn delete_bucket_analytics_configuration(
        &self,
        input: DeleteBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketAnalyticsConfigurationError>;

    /// <p>Deletes the cors configuration information set for the bucket.</p>
    fn delete_bucket_cors(
        &self,
        input: DeleteBucketCorsRequest,
    ) -> RusotoFuture<(), DeleteBucketCorsError>;

    /// <p>Deletes the server-side encryption configuration from the bucket.</p>
    fn delete_bucket_encryption(
        &self,
        input: DeleteBucketEncryptionRequest,
    ) -> RusotoFuture<(), DeleteBucketEncryptionError>;

    /// <p>Deletes an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn delete_bucket_inventory_configuration(
        &self,
        input: DeleteBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketInventoryConfigurationError>;

    /// <p>Deletes the lifecycle configuration from the bucket.</p>
    fn delete_bucket_lifecycle(
        &self,
        input: DeleteBucketLifecycleRequest,
    ) -> RusotoFuture<(), DeleteBucketLifecycleError>;

    /// <p>Deletes a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    fn delete_bucket_metrics_configuration(
        &self,
        input: DeleteBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketMetricsConfigurationError>;

    /// <p>Deletes the policy from the bucket.</p>
    fn delete_bucket_policy(
        &self,
        input: DeleteBucketPolicyRequest,
    ) -> RusotoFuture<(), DeleteBucketPolicyError>;

    /// <p>Deletes the replication configuration from the bucket.</p>
    fn delete_bucket_replication(
        &self,
        input: DeleteBucketReplicationRequest,
    ) -> RusotoFuture<(), DeleteBucketReplicationError>;

    /// <p>Deletes the tags from the bucket.</p>
    fn delete_bucket_tagging(
        &self,
        input: DeleteBucketTaggingRequest,
    ) -> RusotoFuture<(), DeleteBucketTaggingError>;

    /// <p>This operation removes the website configuration from the bucket.</p>
    fn delete_bucket_website(
        &self,
        input: DeleteBucketWebsiteRequest,
    ) -> RusotoFuture<(), DeleteBucketWebsiteError>;

    /// <p>Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects.</p>
    fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectOutput, DeleteObjectError>;

    /// <p>Removes the tag-set from an existing object.</p>
    fn delete_object_tagging(
        &self,
        input: DeleteObjectTaggingRequest,
    ) -> RusotoFuture<DeleteObjectTaggingOutput, DeleteObjectTaggingError>;

    /// <p>This operation enables you to delete multiple objects from a bucket using a single HTTP request. You may specify up to 1000 keys.</p>
    fn delete_objects(
        &self,
        input: DeleteObjectsRequest,
    ) -> RusotoFuture<DeleteObjectsOutput, DeleteObjectsError>;

    /// <p>Returns the accelerate configuration of a bucket.</p>
    fn get_bucket_accelerate_configuration(
        &self,
        input: GetBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError>;

    /// <p>Gets the access control policy for the bucket.</p>
    fn get_bucket_acl(
        &self,
        input: GetBucketAclRequest,
    ) -> RusotoFuture<GetBucketAclOutput, GetBucketAclError>;

    /// <p>Gets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn get_bucket_analytics_configuration(
        &self,
        input: GetBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError>;

    /// <p>Returns the cors configuration for the bucket.</p>
    fn get_bucket_cors(
        &self,
        input: GetBucketCorsRequest,
    ) -> RusotoFuture<GetBucketCorsOutput, GetBucketCorsError>;

    /// <p>Returns the server-side encryption configuration of a bucket.</p>
    fn get_bucket_encryption(
        &self,
        input: GetBucketEncryptionRequest,
    ) -> RusotoFuture<GetBucketEncryptionOutput, GetBucketEncryptionError>;

    /// <p>Returns an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn get_bucket_inventory_configuration(
        &self,
        input: GetBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError>;

    /// <p>Deprecated, see the GetBucketLifecycleConfiguration operation.</p>
    fn get_bucket_lifecycle(
        &self,
        input: GetBucketLifecycleRequest,
    ) -> RusotoFuture<GetBucketLifecycleOutput, GetBucketLifecycleError>;

    /// <p>Returns the lifecycle configuration information set on the bucket.</p>
    fn get_bucket_lifecycle_configuration(
        &self,
        input: GetBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError>;

    /// <p>Returns the region the bucket resides in.</p>
    fn get_bucket_location(
        &self,
        input: GetBucketLocationRequest,
    ) -> RusotoFuture<GetBucketLocationOutput, GetBucketLocationError>;

    /// <p>Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner.</p>
    fn get_bucket_logging(
        &self,
        input: GetBucketLoggingRequest,
    ) -> RusotoFuture<GetBucketLoggingOutput, GetBucketLoggingError>;

    /// <p>Gets a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    fn get_bucket_metrics_configuration(
        &self,
        input: GetBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError>;

    /// <p>Deprecated, see the GetBucketNotificationConfiguration operation.</p>
    fn get_bucket_notification(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfigurationDeprecated, GetBucketNotificationError>;

    /// <p>Returns the notification configuration of a bucket.</p>
    fn get_bucket_notification_configuration(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfiguration, GetBucketNotificationConfigurationError>;

    /// <p>Returns the policy of a specified bucket.</p>
    fn get_bucket_policy(
        &self,
        input: GetBucketPolicyRequest,
    ) -> RusotoFuture<GetBucketPolicyOutput, GetBucketPolicyError>;

    /// <p>Returns the replication configuration of a bucket.</p>
    fn get_bucket_replication(
        &self,
        input: GetBucketReplicationRequest,
    ) -> RusotoFuture<GetBucketReplicationOutput, GetBucketReplicationError>;

    /// <p>Returns the request payment configuration of a bucket.</p>
    fn get_bucket_request_payment(
        &self,
        input: GetBucketRequestPaymentRequest,
    ) -> RusotoFuture<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError>;

    /// <p>Returns the tag set associated with the bucket.</p>
    fn get_bucket_tagging(
        &self,
        input: GetBucketTaggingRequest,
    ) -> RusotoFuture<GetBucketTaggingOutput, GetBucketTaggingError>;

    /// <p>Returns the versioning state of a bucket.</p>
    fn get_bucket_versioning(
        &self,
        input: GetBucketVersioningRequest,
    ) -> RusotoFuture<GetBucketVersioningOutput, GetBucketVersioningError>;

    /// <p>Returns the website configuration for a bucket.</p>
    fn get_bucket_website(
        &self,
        input: GetBucketWebsiteRequest,
    ) -> RusotoFuture<GetBucketWebsiteOutput, GetBucketWebsiteError>;

    /// <p>Retrieves objects from Amazon S3.</p>
    fn get_object(&self, input: GetObjectRequest) -> RusotoFuture<GetObjectOutput, GetObjectError>;

    /// <p>Returns the access control list (ACL) of an object.</p>
    fn get_object_acl(
        &self,
        input: GetObjectAclRequest,
    ) -> RusotoFuture<GetObjectAclOutput, GetObjectAclError>;

    /// <p>Returns the tag-set of an object.</p>
    fn get_object_tagging(
        &self,
        input: GetObjectTaggingRequest,
    ) -> RusotoFuture<GetObjectTaggingOutput, GetObjectTaggingError>;

    /// <p>Return torrent files from a bucket.</p>
    fn get_object_torrent(
        &self,
        input: GetObjectTorrentRequest,
    ) -> RusotoFuture<GetObjectTorrentOutput, GetObjectTorrentError>;

    /// <p>This operation is useful to determine if a bucket exists and you have permission to access it.</p>
    fn head_bucket(&self, input: HeadBucketRequest) -> RusotoFuture<(), HeadBucketError>;

    /// <p>The HEAD operation retrieves metadata from an object without returning the object itself. This operation is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object.</p>
    fn head_object(
        &self,
        input: HeadObjectRequest,
    ) -> RusotoFuture<HeadObjectOutput, HeadObjectError>;

    /// <p>Lists the analytics configurations for the bucket.</p>
    fn list_bucket_analytics_configurations(
        &self,
        input: ListBucketAnalyticsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError>;

    /// <p>Returns a list of inventory configurations for the bucket.</p>
    fn list_bucket_inventory_configurations(
        &self,
        input: ListBucketInventoryConfigurationsRequest,
    ) -> RusotoFuture<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError>;

    /// <p>Lists the metrics configurations for the bucket.</p>
    fn list_bucket_metrics_configurations(
        &self,
        input: ListBucketMetricsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError>;

    /// <p>Returns a list of all buckets owned by the authenticated sender of the request.</p>
    fn list_buckets(&self) -> RusotoFuture<ListBucketsOutput, ListBucketsError>;

    /// <p>This operation lists in-progress multipart uploads.</p>
    fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsRequest,
    ) -> RusotoFuture<ListMultipartUploadsOutput, ListMultipartUploadsError>;

    /// <p>Returns metadata about all of the versions of objects in a bucket.</p>
    fn list_object_versions(
        &self,
        input: ListObjectVersionsRequest,
    ) -> RusotoFuture<ListObjectVersionsOutput, ListObjectVersionsError>;

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket.</p>
    fn list_objects(
        &self,
        input: ListObjectsRequest,
    ) -> RusotoFuture<ListObjectsOutput, ListObjectsError>;

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. Note: ListObjectsV2 is the revised List Objects API and we recommend you use this revised API for new application development.</p>
    fn list_objects_v2(
        &self,
        input: ListObjectsV2Request,
    ) -> RusotoFuture<ListObjectsV2Output, ListObjectsV2Error>;

    /// <p>Lists the parts that have been uploaded for a specific multipart upload.</p>
    fn list_parts(&self, input: ListPartsRequest) -> RusotoFuture<ListPartsOutput, ListPartsError>;

    /// <p>Sets the accelerate configuration of an existing bucket.</p>
    fn put_bucket_accelerate_configuration(
        &self,
        input: PutBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAccelerateConfigurationError>;

    /// <p>Sets the permissions on a bucket using access control lists (ACL).</p>
    fn put_bucket_acl(&self, input: PutBucketAclRequest) -> RusotoFuture<(), PutBucketAclError>;

    /// <p>Sets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn put_bucket_analytics_configuration(
        &self,
        input: PutBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAnalyticsConfigurationError>;

    /// <p>Sets the cors configuration for a bucket.</p>
    fn put_bucket_cors(&self, input: PutBucketCorsRequest) -> RusotoFuture<(), PutBucketCorsError>;

    /// <p>Creates a new server-side encryption configuration (or replaces an existing one, if present).</p>
    fn put_bucket_encryption(
        &self,
        input: PutBucketEncryptionRequest,
    ) -> RusotoFuture<(), PutBucketEncryptionError>;

    /// <p>Adds an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn put_bucket_inventory_configuration(
        &self,
        input: PutBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketInventoryConfigurationError>;

    /// <p>Deprecated, see the PutBucketLifecycleConfiguration operation.</p>
    fn put_bucket_lifecycle(
        &self,
        input: PutBucketLifecycleRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleError>;

    /// <p>Sets lifecycle configuration for your bucket. If a lifecycle configuration exists, it replaces it.</p>
    fn put_bucket_lifecycle_configuration(
        &self,
        input: PutBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleConfigurationError>;

    /// <p>Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. To set the logging status of a bucket, you must be the bucket owner.</p>
    fn put_bucket_logging(
        &self,
        input: PutBucketLoggingRequest,
    ) -> RusotoFuture<(), PutBucketLoggingError>;

    /// <p>Sets a metrics configuration (specified by the metrics configuration ID) for the bucket.</p>
    fn put_bucket_metrics_configuration(
        &self,
        input: PutBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketMetricsConfigurationError>;

    /// <p>Deprecated, see the PutBucketNotificationConfiguraiton operation.</p>
    fn put_bucket_notification(
        &self,
        input: PutBucketNotificationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationError>;

    /// <p>Enables notifications of specified events for a bucket.</p>
    fn put_bucket_notification_configuration(
        &self,
        input: PutBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationConfigurationError>;

    /// <p>Replaces a policy on a bucket. If the bucket already has a policy, the one in this request completely replaces it.</p>
    fn put_bucket_policy(
        &self,
        input: PutBucketPolicyRequest,
    ) -> RusotoFuture<(), PutBucketPolicyError>;

    /// <p>Creates a new replication configuration (or replaces an existing one, if present).</p>
    fn put_bucket_replication(
        &self,
        input: PutBucketReplicationRequest,
    ) -> RusotoFuture<(), PutBucketReplicationError>;

    /// <p>Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. Documentation on requester pays buckets can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html</p>
    fn put_bucket_request_payment(
        &self,
        input: PutBucketRequestPaymentRequest,
    ) -> RusotoFuture<(), PutBucketRequestPaymentError>;

    /// <p>Sets the tags for a bucket.</p>
    fn put_bucket_tagging(
        &self,
        input: PutBucketTaggingRequest,
    ) -> RusotoFuture<(), PutBucketTaggingError>;

    /// <p>Sets the versioning state of an existing bucket. To set the versioning state, you must be the bucket owner.</p>
    fn put_bucket_versioning(
        &self,
        input: PutBucketVersioningRequest,
    ) -> RusotoFuture<(), PutBucketVersioningError>;

    /// <p>Set the website configuration for a bucket.</p>
    fn put_bucket_website(
        &self,
        input: PutBucketWebsiteRequest,
    ) -> RusotoFuture<(), PutBucketWebsiteError>;

    /// <p>Adds an object to a bucket.</p>
    fn put_object(&self, input: PutObjectRequest) -> RusotoFuture<PutObjectOutput, PutObjectError>;

    /// <p>uses the acl subresource to set the access control list (ACL) permissions for an object that already exists in a bucket</p>
    fn put_object_acl(
        &self,
        input: PutObjectAclRequest,
    ) -> RusotoFuture<PutObjectAclOutput, PutObjectAclError>;

    /// <p>Sets the supplied tag-set to an object that already exists in a bucket</p>
    fn put_object_tagging(
        &self,
        input: PutObjectTaggingRequest,
    ) -> RusotoFuture<PutObjectTaggingOutput, PutObjectTaggingError>;

    /// <p>Restores an archived copy of an object back into Amazon S3</p>
    fn restore_object(
        &self,
        input: RestoreObjectRequest,
    ) -> RusotoFuture<RestoreObjectOutput, RestoreObjectError>;

    /// <p>This operation filters the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must also specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records, and returns only records that match the specified SQL expression. You must also specify the data serialization format for the response.</p>
    fn select_object_content(
        &self,
        input: SelectObjectContentRequest,
    ) -> RusotoFuture<SelectObjectContentOutput, SelectObjectContentError>;

    /// <p>Uploads a part in a multipart upload.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    fn upload_part(
        &self,
        input: UploadPartRequest,
    ) -> RusotoFuture<UploadPartOutput, UploadPartError>;

    /// <p>Uploads a part by copying data from an existing object as data source.</p>
    fn upload_part_copy(
        &self,
        input: UploadPartCopyRequest,
    ) -> RusotoFuture<UploadPartCopyOutput, UploadPartCopyError>;
}
/// A client for the Amazon S3 API.
pub struct S3Client {
    client: Client,
    region: region::Region,
}

impl S3Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> S3Client {
        S3Client {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> S3Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        S3Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl S3 for S3Client {
    /// <p>Aborts a multipart upload.</p> <p>To verify that all parts have been removed, so you don't get charged for the part storage, you should call the List Parts operation and ensure the parts list is empty.</p>
    #[allow(unused_variables, warnings)]
    fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadRequest,
    ) -> RusotoFuture<AbortMultipartUploadOutput, AbortMultipartUploadError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put("uploadId", &input.upload_id);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AbortMultipartUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = AbortMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(AbortMultipartUploadOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Completes a multipart upload by assembling previously uploaded parts.</p>
    #[allow(unused_variables, warnings)]
    fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadRequest,
    ) -> RusotoFuture<CompleteMultipartUploadOutput, CompleteMultipartUploadError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put("uploadId", &input.upload_id);
        request.set_params(params);
        if input.multipart_upload.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            CompletedMultipartUploadSerializer::serialize(
                &mut writer,
                "CompleteMultipartUpload",
                input.multipart_upload.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CompleteMultipartUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CompleteMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CompleteMultipartUploadOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Creates a copy of an object that is already stored in Amazon S3.</p>
    #[allow(unused_variables, warnings)]
    fn copy_object(
        &self,
        input: CopyObjectRequest,
    ) -> RusotoFuture<CopyObjectOutput, CopyObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = input.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = input.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = input.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }
        request.add_header("x-amz-copy-source", &input.copy_source);

        if let Some(ref copy_source_if_match) = input.copy_source_if_match {
            request.add_header(
                "x-amz-copy-source-if-match",
                &copy_source_if_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_modified_since) = input.copy_source_if_modified_since {
            request.add_header(
                "x-amz-copy-source-if-modified-since",
                &copy_source_if_modified_since.to_string(),
            );
        }

        if let Some(ref copy_source_if_none_match) = input.copy_source_if_none_match {
            request.add_header(
                "x-amz-copy-source-if-none-match",
                &copy_source_if_none_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
            request.add_header(
                "x-amz-copy-source-if-unmodified-since",
                &copy_source_if_unmodified_since.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_algorithm) =
            input.copy_source_sse_customer_algorithm
        {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-algorithm",
                &copy_source_sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key",
                &copy_source_sse_customer_key.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key-MD5",
                &copy_source_sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref expires) = input.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref metadata) = input.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref metadata_directive) = input.metadata_directive {
            request.add_header("x-amz-metadata-directive", &metadata_directive.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = input.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref tagging_directive) = input.tagging_directive {
            request.add_header("x-amz-tagging-directive", &tagging_directive.to_string());
        }

        if let Some(ref website_redirect_location) = input.website_redirect_location {
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CopyObjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CopyObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CopyObjectOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(copy_source_version_id) =
                    response.headers.get("x-amz-copy-source-version-id")
                {
                    let value = copy_source_version_id.to_owned();
                    result.copy_source_version_id = Some(value)
                };
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Creates a new bucket.</p>
    #[allow(unused_variables, warnings)]
    fn create_bucket(
        &self,
        input: CreateBucketRequest,
    ) -> RusotoFuture<CreateBucketOutput, CreateBucketError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = input.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if input.create_bucket_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            CreateBucketConfigurationSerializer::serialize(
                &mut writer,
                "CreateBucketConfiguration",
                input.create_bucket_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateBucketError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateBucketOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateBucketOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Initiates a multipart upload and returns an upload ID.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    #[allow(unused_variables, warnings)]
    fn create_multipart_upload(
        &self,
        input: CreateMultipartUploadRequest,
    ) -> RusotoFuture<CreateMultipartUploadOutput, CreateMultipartUploadError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);

        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = input.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = input.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = input.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        if let Some(ref expires) = input.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref metadata) = input.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = input.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref website_redirect_location) = input.website_redirect_location {
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }
        let mut params = Params::new();
        params.put_key("uploads");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateMultipartUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateMultipartUploadOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(abort_date) = response.headers.get("x-amz-abort-date") {
                    let value = abort_date.to_owned();
                    result.abort_date = Some(value)
                };
                if let Some(abort_rule_id) = response.headers.get("x-amz-abort-rule-id") {
                    let value = abort_rule_id.to_owned();
                    result.abort_rule_id = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Deletes the bucket. All objects (including all object versions and Delete Markers) in the bucket must be deleted before the bucket itself can be deleted.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket(&self, input: DeleteBucketRequest) -> RusotoFuture<(), DeleteBucketError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_analytics_configuration(
        &self,
        input: DeleteBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketAnalyticsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketAnalyticsConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the cors configuration information set for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_cors(
        &self,
        input: DeleteBucketCorsRequest,
    ) -> RusotoFuture<(), DeleteBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketCorsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the server-side encryption configuration from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_encryption(
        &self,
        input: DeleteBucketEncryptionRequest,
    ) -> RusotoFuture<(), DeleteBucketEncryptionError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketEncryptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes an inventory configuration (identified by the inventory ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_inventory_configuration(
        &self,
        input: DeleteBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketInventoryConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketInventoryConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the lifecycle configuration from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_lifecycle(
        &self,
        input: DeleteBucketLifecycleRequest,
    ) -> RusotoFuture<(), DeleteBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketLifecycleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_metrics_configuration(
        &self,
        input: DeleteBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketMetricsConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the policy from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_policy(
        &self,
        input: DeleteBucketPolicyRequest,
    ) -> RusotoFuture<(), DeleteBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the replication configuration from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_replication(
        &self,
        input: DeleteBucketReplicationRequest,
    ) -> RusotoFuture<(), DeleteBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketReplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the tags from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_tagging(
        &self,
        input: DeleteBucketTaggingRequest,
    ) -> RusotoFuture<(), DeleteBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketTaggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>This operation removes the website configuration from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_website(
        &self,
        input: DeleteBucketWebsiteRequest,
    ) -> RusotoFuture<(), DeleteBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketWebsiteError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects.</p>
    #[allow(unused_variables, warnings)]
    fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectOutput, DeleteObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        if let Some(ref mfa) = input.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteObjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteObjectOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(delete_marker) = response.headers.get("x-amz-delete-marker") {
                    let value = delete_marker.to_owned();
                    result.delete_marker = Some(value.parse::<bool>().unwrap())
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Removes the tag-set from an existing object.</p>
    #[allow(unused_variables, warnings)]
    fn delete_object_tagging(
        &self,
        input: DeleteObjectTaggingRequest,
    ) -> RusotoFuture<DeleteObjectTaggingOutput, DeleteObjectTaggingError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteObjectTaggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteObjectTaggingOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>This operation enables you to delete multiple objects from a bucket using a single HTTP request. You may specify up to 1000 keys.</p>
    #[allow(unused_variables, warnings)]
    fn delete_objects(
        &self,
        input: DeleteObjectsRequest,
    ) -> RusotoFuture<DeleteObjectsOutput, DeleteObjectsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);

        if let Some(ref mfa) = input.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put_key("delete");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        DeleteSerializer::serialize(&mut writer, "Delete", &input.delete);
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteObjectsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteObjectsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteObjectsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Returns the accelerate configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_accelerate_configuration(
        &self,
        input: GetBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("accelerate");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketAccelerateConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketAccelerateConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetBucketAccelerateConfigurationOutputDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Gets the access control policy for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_acl(
        &self,
        input: GetBucketAclRequest,
    ) -> RusotoFuture<GetBucketAclOutput, GetBucketAclError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("acl");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketAclError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketAclOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Gets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_analytics_configuration(
        &self,
        input: GetBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketAnalyticsConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketAnalyticsConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetBucketAnalyticsConfigurationOutputDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the cors configuration for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_cors(
        &self,
        input: GetBucketCorsRequest,
    ) -> RusotoFuture<GetBucketCorsOutput, GetBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketCorsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketCorsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketCorsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the server-side encryption configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_encryption(
        &self,
        input: GetBucketEncryptionRequest,
    ) -> RusotoFuture<GetBucketEncryptionOutput, GetBucketEncryptionError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketEncryptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketEncryptionOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketEncryptionOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns an inventory configuration (identified by the inventory ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_inventory_configuration(
        &self,
        input: GetBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketInventoryConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketInventoryConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetBucketInventoryConfigurationOutputDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Deprecated, see the GetBucketLifecycleConfiguration operation.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_lifecycle(
        &self,
        input: GetBucketLifecycleRequest,
    ) -> RusotoFuture<GetBucketLifecycleOutput, GetBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketLifecycleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketLifecycleOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketLifecycleOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the lifecycle configuration information set on the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_lifecycle_configuration(
        &self,
        input: GetBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketLifecycleConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketLifecycleConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetBucketLifecycleConfigurationOutputDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the region the bucket resides in.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_location(
        &self,
        input: GetBucketLocationRequest,
    ) -> RusotoFuture<GetBucketLocationOutput, GetBucketLocationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("location");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketLocationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketLocationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketLocationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_logging(
        &self,
        input: GetBucketLoggingRequest,
    ) -> RusotoFuture<GetBucketLoggingOutput, GetBucketLoggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("logging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketLoggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketLoggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketLoggingOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Gets a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_metrics_configuration(
        &self,
        input: GetBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketMetricsConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketMetricsConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetBucketMetricsConfigurationOutputDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Deprecated, see the GetBucketNotificationConfiguration operation.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_notification(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfigurationDeprecated, GetBucketNotificationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketNotificationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = NotificationConfigurationDeprecated::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        NotificationConfigurationDeprecatedDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the notification configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_notification_configuration(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfiguration, GetBucketNotificationConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketNotificationConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = NotificationConfiguration::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(NotificationConfigurationDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the policy of a specified bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_policy(
        &self,
        input: GetBucketPolicyRequest,
    ) -> RusotoFuture<GetBucketPolicyOutput, GetBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().map(move |response| {
                let mut result = GetBucketPolicyOutput::default();
                result.policy = Some(String::from_utf8_lossy(response.body.as_ref()).into());

                result
            }))
        })
    }

    /// <p>Returns the replication configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_replication(
        &self,
        input: GetBucketReplicationRequest,
    ) -> RusotoFuture<GetBucketReplicationOutput, GetBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketReplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketReplicationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketReplicationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the request payment configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_request_payment(
        &self,
        input: GetBucketRequestPaymentRequest,
    ) -> RusotoFuture<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("requestPayment");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketRequestPaymentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketRequestPaymentOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketRequestPaymentOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the tag set associated with the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_tagging(
        &self,
        input: GetBucketTaggingRequest,
    ) -> RusotoFuture<GetBucketTaggingOutput, GetBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketTaggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketTaggingOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the versioning state of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_versioning(
        &self,
        input: GetBucketVersioningRequest,
    ) -> RusotoFuture<GetBucketVersioningOutput, GetBucketVersioningError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("versioning");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketVersioningError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketVersioningOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketVersioningOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns the website configuration for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_website(
        &self,
        input: GetBucketWebsiteRequest,
    ) -> RusotoFuture<GetBucketWebsiteOutput, GetBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketWebsiteError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketWebsiteOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketWebsiteOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Retrieves objects from Amazon S3.</p>
    #[allow(unused_variables, warnings)]
    fn get_object(&self, input: GetObjectRequest) -> RusotoFuture<GetObjectOutput, GetObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        if let Some(ref if_modified_since) = input.if_modified_since {
            request.add_header("If-Modified-Since", &if_modified_since.to_string());
        }

        if let Some(ref if_none_match) = input.if_none_match {
            request.add_header("If-None-Match", &if_none_match.to_string());
        }

        if let Some(ref if_unmodified_since) = input.if_unmodified_since {
            request.add_header("If-Unmodified-Since", &if_unmodified_since.to_string());
        }

        if let Some(ref range) = input.range {
            request.add_header("Range", &range.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        if let Some(ref x) = input.part_number {
            params.put("partNumber", x);
        }
        if let Some(ref x) = input.response_cache_control {
            params.put("response-cache-control", x);
        }
        if let Some(ref x) = input.response_content_disposition {
            params.put("response-content-disposition", x);
        }
        if let Some(ref x) = input.response_content_encoding {
            params.put("response-content-encoding", x);
        }
        if let Some(ref x) = input.response_content_language {
            params.put("response-content-language", x);
        }
        if let Some(ref x) = input.response_content_type {
            params.put("response-content-type", x);
        }
        if let Some(ref x) = input.response_expires {
            params.put("response-expires", x);
        }
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetObjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            let mut result = GetObjectOutput::default();
            result.body = Some(StreamingBody {
                len: None,
                inner: response.body,
            });
            if let Some(accept_ranges) = response.headers.get("accept-ranges") {
                let value = accept_ranges.to_owned();
                result.accept_ranges = Some(value)
            };
            if let Some(cache_control) = response.headers.get("Cache-Control") {
                let value = cache_control.to_owned();
                result.cache_control = Some(value)
            };
            if let Some(content_disposition) = response.headers.get("Content-Disposition") {
                let value = content_disposition.to_owned();
                result.content_disposition = Some(value)
            };
            if let Some(content_encoding) = response.headers.get("Content-Encoding") {
                let value = content_encoding.to_owned();
                result.content_encoding = Some(value)
            };
            if let Some(content_language) = response.headers.get("Content-Language") {
                let value = content_language.to_owned();
                result.content_language = Some(value)
            };
            if let Some(content_length) = response.headers.get("Content-Length") {
                let value = content_length.to_owned();
                result.content_length = Some(value.parse::<i64>().unwrap())
            };
            if let Some(content_range) = response.headers.get("Content-Range") {
                let value = content_range.to_owned();
                result.content_range = Some(value)
            };
            if let Some(content_type) = response.headers.get("Content-Type") {
                let value = content_type.to_owned();
                result.content_type = Some(value)
            };
            if let Some(delete_marker) = response.headers.get("x-amz-delete-marker") {
                let value = delete_marker.to_owned();
                result.delete_marker = Some(value.parse::<bool>().unwrap())
            };
            if let Some(e_tag) = response.headers.get("ETag") {
                let value = e_tag.to_owned();
                result.e_tag = Some(value)
            };
            if let Some(expiration) = response.headers.get("x-amz-expiration") {
                let value = expiration.to_owned();
                result.expiration = Some(value)
            };
            if let Some(expires) = response.headers.get("Expires") {
                let value = expires.to_owned();
                result.expires = Some(value)
            };
            if let Some(last_modified) = response.headers.get("Last-Modified") {
                let value = last_modified.to_owned();
                result.last_modified = Some(value)
            };
            let mut values = ::std::collections::HashMap::new();
            for (key, value) in response.headers.iter() {
                if key.starts_with("x-amz-meta-") {
                    values.insert(key["x-amz-meta-".len()..].to_owned(), value.to_owned());
                }
            }
            result.metadata = Some(values);
            if let Some(missing_meta) = response.headers.get("x-amz-missing-meta") {
                let value = missing_meta.to_owned();
                result.missing_meta = Some(value.parse::<i64>().unwrap())
            };
            if let Some(parts_count) = response.headers.get("x-amz-mp-parts-count") {
                let value = parts_count.to_owned();
                result.parts_count = Some(value.parse::<i64>().unwrap())
            };
            if let Some(replication_status) = response.headers.get("x-amz-replication-status") {
                let value = replication_status.to_owned();
                result.replication_status = Some(value)
            };
            if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                let value = request_charged.to_owned();
                result.request_charged = Some(value)
            };
            if let Some(restore) = response.headers.get("x-amz-restore") {
                let value = restore.to_owned();
                result.restore = Some(value)
            };
            if let Some(sse_customer_algorithm) = response
                .headers
                .get("x-amz-server-side-encryption-customer-algorithm")
            {
                let value = sse_customer_algorithm.to_owned();
                result.sse_customer_algorithm = Some(value)
            };
            if let Some(sse_customer_key_md5) = response
                .headers
                .get("x-amz-server-side-encryption-customer-key-MD5")
            {
                let value = sse_customer_key_md5.to_owned();
                result.sse_customer_key_md5 = Some(value)
            };
            if let Some(ssekms_key_id) = response
                .headers
                .get("x-amz-server-side-encryption-aws-kms-key-id")
            {
                let value = ssekms_key_id.to_owned();
                result.ssekms_key_id = Some(value)
            };
            if let Some(server_side_encryption) =
                response.headers.get("x-amz-server-side-encryption")
            {
                let value = server_side_encryption.to_owned();
                result.server_side_encryption = Some(value)
            };
            if let Some(storage_class) = response.headers.get("x-amz-storage-class") {
                let value = storage_class.to_owned();
                result.storage_class = Some(value)
            };
            if let Some(tag_count) = response.headers.get("x-amz-tagging-count") {
                let value = tag_count.to_owned();
                result.tag_count = Some(value.parse::<i64>().unwrap())
            };
            if let Some(version_id) = response.headers.get("x-amz-version-id") {
                let value = version_id.to_owned();
                result.version_id = Some(value)
            };
            if let Some(website_redirect_location) =
                response.headers.get("x-amz-website-redirect-location")
            {
                let value = website_redirect_location.to_owned();
                result.website_redirect_location = Some(value)
            };
            Box::new(future::ok(result))
        })
    }

    /// <p>Returns the access control list (ACL) of an object.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_acl(
        &self,
        input: GetObjectAclRequest,
    ) -> RusotoFuture<GetObjectAclOutput, GetObjectAclError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("acl");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetObjectAclError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetObjectAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetObjectAclOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Returns the tag-set of an object.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_tagging(
        &self,
        input: GetObjectTaggingRequest,
    ) -> RusotoFuture<GetObjectTaggingOutput, GetObjectTaggingError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetObjectTaggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetObjectTaggingOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Return torrent files from a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_torrent(
        &self,
        input: GetObjectTorrentRequest,
    ) -> RusotoFuture<GetObjectTorrentOutput, GetObjectTorrentError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put_key("torrent");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetObjectTorrentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            let mut result = GetObjectTorrentOutput::default();
            result.body = Some(StreamingBody {
                len: None,
                inner: response.body,
            });
            if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                let value = request_charged.to_owned();
                result.request_charged = Some(value)
            };
            Box::new(future::ok(result))
        })
    }

    /// <p>This operation is useful to determine if a bucket exists and you have permission to access it.</p>
    #[allow(unused_variables, warnings)]
    fn head_bucket(&self, input: HeadBucketRequest) -> RusotoFuture<(), HeadBucketError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("HEAD", "s3", &self.region, &request_uri);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(HeadBucketError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>The HEAD operation retrieves metadata from an object without returning the object itself. This operation is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object.</p>
    #[allow(unused_variables, warnings)]
    fn head_object(
        &self,
        input: HeadObjectRequest,
    ) -> RusotoFuture<HeadObjectOutput, HeadObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("HEAD", "s3", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        if let Some(ref if_modified_since) = input.if_modified_since {
            request.add_header("If-Modified-Since", &if_modified_since.to_string());
        }

        if let Some(ref if_none_match) = input.if_none_match {
            request.add_header("If-None-Match", &if_none_match.to_string());
        }

        if let Some(ref if_unmodified_since) = input.if_unmodified_since {
            request.add_header("If-Unmodified-Since", &if_unmodified_since.to_string());
        }

        if let Some(ref range) = input.range {
            request.add_header("Range", &range.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        if let Some(ref x) = input.part_number {
            params.put("partNumber", x);
        }
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(HeadObjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = HeadObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(HeadObjectOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(accept_ranges) = response.headers.get("accept-ranges") {
                    let value = accept_ranges.to_owned();
                    result.accept_ranges = Some(value)
                };
                if let Some(cache_control) = response.headers.get("Cache-Control") {
                    let value = cache_control.to_owned();
                    result.cache_control = Some(value)
                };
                if let Some(content_disposition) = response.headers.get("Content-Disposition") {
                    let value = content_disposition.to_owned();
                    result.content_disposition = Some(value)
                };
                if let Some(content_encoding) = response.headers.get("Content-Encoding") {
                    let value = content_encoding.to_owned();
                    result.content_encoding = Some(value)
                };
                if let Some(content_language) = response.headers.get("Content-Language") {
                    let value = content_language.to_owned();
                    result.content_language = Some(value)
                };
                if let Some(content_length) = response.headers.get("Content-Length") {
                    let value = content_length.to_owned();
                    result.content_length = Some(value.parse::<i64>().unwrap())
                };
                if let Some(content_type) = response.headers.get("Content-Type") {
                    let value = content_type.to_owned();
                    result.content_type = Some(value)
                };
                if let Some(delete_marker) = response.headers.get("x-amz-delete-marker") {
                    let value = delete_marker.to_owned();
                    result.delete_marker = Some(value.parse::<bool>().unwrap())
                };
                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(expires) = response.headers.get("Expires") {
                    let value = expires.to_owned();
                    result.expires = Some(value)
                };
                if let Some(last_modified) = response.headers.get("Last-Modified") {
                    let value = last_modified.to_owned();
                    result.last_modified = Some(value)
                };
                let mut values = ::std::collections::HashMap::new();
                for (key, value) in response.headers.iter() {
                    if key.starts_with("x-amz-meta-") {
                        values.insert(key["x-amz-meta-".len()..].to_owned(), value.to_owned());
                    }
                }
                result.metadata = Some(values);
                if let Some(missing_meta) = response.headers.get("x-amz-missing-meta") {
                    let value = missing_meta.to_owned();
                    result.missing_meta = Some(value.parse::<i64>().unwrap())
                };
                if let Some(parts_count) = response.headers.get("x-amz-mp-parts-count") {
                    let value = parts_count.to_owned();
                    result.parts_count = Some(value.parse::<i64>().unwrap())
                };
                if let Some(replication_status) = response.headers.get("x-amz-replication-status") {
                    let value = replication_status.to_owned();
                    result.replication_status = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(restore) = response.headers.get("x-amz-restore") {
                    let value = restore.to_owned();
                    result.restore = Some(value)
                };
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(storage_class) = response.headers.get("x-amz-storage-class") {
                    let value = storage_class.to_owned();
                    result.storage_class = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                if let Some(website_redirect_location) =
                    response.headers.get("x-amz-website-redirect-location")
                {
                    let value = website_redirect_location.to_owned();
                    result.website_redirect_location = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Lists the analytics configurations for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_bucket_analytics_configurations(
        &self,
        input: ListBucketAnalyticsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("analytics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBucketAnalyticsConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListBucketAnalyticsConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        ListBucketAnalyticsConfigurationsOutputDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of inventory configurations for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_bucket_inventory_configurations(
        &self,
        input: ListBucketInventoryConfigurationsRequest,
    ) -> RusotoFuture<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("inventory");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBucketInventoryConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListBucketInventoryConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        ListBucketInventoryConfigurationsOutputDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Lists the metrics configurations for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_bucket_metrics_configurations(
        &self,
        input: ListBucketMetricsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("metrics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBucketMetricsConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListBucketMetricsConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        ListBucketMetricsConfigurationsOutputDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of all buckets owned by the authenticated sender of the request.</p>
    #[allow(unused_variables, warnings)]
    fn list_buckets(&self) -> RusotoFuture<ListBucketsOutput, ListBucketsError> {
        let request_uri = "/";

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBucketsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListBucketsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListBucketsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>This operation lists in-progress multipart uploads.</p>
    #[allow(unused_variables, warnings)]
    fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsRequest,
    ) -> RusotoFuture<ListMultipartUploadsOutput, ListMultipartUploadsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = input.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = input.key_marker {
            params.put("key-marker", x);
        }
        if let Some(ref x) = input.max_uploads {
            params.put("max-uploads", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = input.upload_id_marker {
            params.put("upload-id-marker", x);
        }
        params.put_key("uploads");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListMultipartUploadsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListMultipartUploadsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListMultipartUploadsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns metadata about all of the versions of objects in a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_object_versions(
        &self,
        input: ListObjectVersionsRequest,
    ) -> RusotoFuture<ListObjectVersionsOutput, ListObjectVersionsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = input.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = input.key_marker {
            params.put("key-marker", x);
        }
        if let Some(ref x) = input.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = input.version_id_marker {
            params.put("version-id-marker", x);
        }
        params.put_key("versions");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListObjectVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListObjectVersionsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListObjectVersionsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_objects(
        &self,
        input: ListObjectsRequest,
    ) -> RusotoFuture<ListObjectsOutput, ListObjectsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = input.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListObjectsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListObjectsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListObjectsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. Note: ListObjectsV2 is the revised List Objects API and we recommend you use this revised API for new application development.</p>
    #[allow(unused_variables, warnings)]
    fn list_objects_v2(
        &self,
        input: ListObjectsV2Request,
    ) -> RusotoFuture<ListObjectsV2Output, ListObjectsV2Error> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        if let Some(ref x) = input.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = input.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = input.fetch_owner {
            params.put("fetch-owner", x);
        }
        if let Some(ref x) = input.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = input.start_after {
            params.put("start-after", x);
        }
        params.put("list-type", "2");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListObjectsV2Error::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListObjectsV2Output::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListObjectsV2OutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Lists the parts that have been uploaded for a specific multipart upload.</p>
    #[allow(unused_variables, warnings)]
    fn list_parts(&self, input: ListPartsRequest) -> RusotoFuture<ListPartsOutput, ListPartsError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.max_parts {
            params.put("max-parts", x);
        }
        if let Some(ref x) = input.part_number_marker {
            params.put("part-number-marker", x);
        }
        params.put("uploadId", &input.upload_id);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListPartsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListPartsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListPartsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(abort_date) = response.headers.get("x-amz-abort-date") {
                    let value = abort_date.to_owned();
                    result.abort_date = Some(value)
                };
                if let Some(abort_rule_id) = response.headers.get("x-amz-abort-rule-id") {
                    let value = abort_rule_id.to_owned();
                    result.abort_rule_id = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Sets the accelerate configuration of an existing bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_accelerate_configuration(
        &self,
        input: PutBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAccelerateConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("accelerate");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        AccelerateConfigurationSerializer::serialize(
            &mut writer,
            "AccelerateConfiguration",
            &input.accelerate_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketAccelerateConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the permissions on a bucket using access control lists (ACL).</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_acl(&self, input: PutBucketAclRequest) -> RusotoFuture<(), PutBucketAclError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = input.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }
        let mut params = Params::new();
        params.put_key("acl");
        request.set_params(params);
        if input.access_control_policy.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            AccessControlPolicySerializer::serialize(
                &mut writer,
                "AccessControlPolicy",
                input.access_control_policy.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketAclError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_analytics_configuration(
        &self,
        input: PutBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAnalyticsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        AnalyticsConfigurationSerializer::serialize(
            &mut writer,
            "AnalyticsConfiguration",
            &input.analytics_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketAnalyticsConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the cors configuration for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_cors(&self, input: PutBucketCorsRequest) -> RusotoFuture<(), PutBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        CORSConfigurationSerializer::serialize(
            &mut writer,
            "CORSConfiguration",
            &input.cors_configuration,
        );
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketCorsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Creates a new server-side encryption configuration (or replaces an existing one, if present).</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_encryption(
        &self,
        input: PutBucketEncryptionRequest,
    ) -> RusotoFuture<(), PutBucketEncryptionError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        ServerSideEncryptionConfigurationSerializer::serialize(
            &mut writer,
            "ServerSideEncryptionConfiguration",
            &input.server_side_encryption_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketEncryptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Adds an inventory configuration (identified by the inventory ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_inventory_configuration(
        &self,
        input: PutBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketInventoryConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        InventoryConfigurationSerializer::serialize(
            &mut writer,
            "InventoryConfiguration",
            &input.inventory_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketInventoryConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deprecated, see the PutBucketLifecycleConfiguration operation.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_lifecycle(
        &self,
        input: PutBucketLifecycleRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);
        if input.lifecycle_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            LifecycleConfigurationSerializer::serialize(
                &mut writer,
                "LifecycleConfiguration",
                input.lifecycle_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketLifecycleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets lifecycle configuration for your bucket. If a lifecycle configuration exists, it replaces it.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_lifecycle_configuration(
        &self,
        input: PutBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);
        if input.lifecycle_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            BucketLifecycleConfigurationSerializer::serialize(
                &mut writer,
                "LifecycleConfiguration",
                input.lifecycle_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketLifecycleConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. To set the logging status of a bucket, you must be the bucket owner.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_logging(
        &self,
        input: PutBucketLoggingRequest,
    ) -> RusotoFuture<(), PutBucketLoggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("logging");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        BucketLoggingStatusSerializer::serialize(
            &mut writer,
            "BucketLoggingStatus",
            &input.bucket_logging_status,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketLoggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets a metrics configuration (specified by the metrics configuration ID) for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_metrics_configuration(
        &self,
        input: PutBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        MetricsConfigurationSerializer::serialize(
            &mut writer,
            "MetricsConfiguration",
            &input.metrics_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketMetricsConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deprecated, see the PutBucketNotificationConfiguraiton operation.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_notification(
        &self,
        input: PutBucketNotificationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        NotificationConfigurationDeprecatedSerializer::serialize(
            &mut writer,
            "NotificationConfigurationDeprecated",
            &input.notification_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketNotificationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Enables notifications of specified events for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_notification_configuration(
        &self,
        input: PutBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        NotificationConfigurationSerializer::serialize(
            &mut writer,
            "NotificationConfiguration",
            &input.notification_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketNotificationConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Replaces a policy on a bucket. If the bucket already has a policy, the one in this request completely replaces it.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_policy(
        &self,
        input: PutBucketPolicyRequest,
    ) -> RusotoFuture<(), PutBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref confirm_remove_self_bucket_access) = input.confirm_remove_self_bucket_access
        {
            request.add_header(
                "x-amz-confirm-remove-self-bucket-access",
                &confirm_remove_self_bucket_access.to_string(),
            );
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        PolicySerializer::serialize(&mut writer, "Policy", &input.policy);
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Creates a new replication configuration (or replaces an existing one, if present).</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_replication(
        &self,
        input: PutBucketReplicationRequest,
    ) -> RusotoFuture<(), PutBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        ReplicationConfigurationSerializer::serialize(
            &mut writer,
            "ReplicationConfiguration",
            &input.replication_configuration,
        );
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketReplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. Documentation on requester pays buckets can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_request_payment(
        &self,
        input: PutBucketRequestPaymentRequest,
    ) -> RusotoFuture<(), PutBucketRequestPaymentError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("requestPayment");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        RequestPaymentConfigurationSerializer::serialize(
            &mut writer,
            "RequestPaymentConfiguration",
            &input.request_payment_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketRequestPaymentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the tags for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_tagging(
        &self,
        input: PutBucketTaggingRequest,
    ) -> RusotoFuture<(), PutBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TaggingSerializer::serialize(&mut writer, "Tagging", &input.tagging);
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketTaggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the versioning state of an existing bucket. To set the versioning state, you must be the bucket owner.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_versioning(
        &self,
        input: PutBucketVersioningRequest,
    ) -> RusotoFuture<(), PutBucketVersioningError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref mfa) = input.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }
        let mut params = Params::new();
        params.put_key("versioning");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        VersioningConfigurationSerializer::serialize(
            &mut writer,
            "VersioningConfiguration",
            &input.versioning_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketVersioningError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Set the website configuration for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_website(
        &self,
        input: PutBucketWebsiteRequest,
    ) -> RusotoFuture<(), PutBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        WebsiteConfigurationSerializer::serialize(
            &mut writer,
            "WebsiteConfiguration",
            &input.website_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketWebsiteError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Adds an object to a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_object(&self, input: PutObjectRequest) -> RusotoFuture<PutObjectOutput, PutObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = input.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = input.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = input.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_length) = input.content_length {
            request.add_header("Content-Length", &content_length.to_string());
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        if let Some(ref expires) = input.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref metadata) = input.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = input.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref website_redirect_location) = input.website_redirect_location {
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }

        if let Some(__body) = input.body {
            request.set_payload_stream(__body.len, __body.inner);
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutObjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(PutObjectOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>uses the acl subresource to set the access control list (ACL) permissions for an object that already exists in a bucket</p>
    #[allow(unused_variables, warnings)]
    fn put_object_acl(
        &self,
        input: PutObjectAclRequest,
    ) -> RusotoFuture<PutObjectAclOutput, PutObjectAclError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = input.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("acl");
        request.set_params(params);
        if input.access_control_policy.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            AccessControlPolicySerializer::serialize(
                &mut writer,
                "AccessControlPolicy",
                input.access_control_policy.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutObjectAclError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(PutObjectAclOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Sets the supplied tag-set to an object that already exists in a bucket</p>
    #[allow(unused_variables, warnings)]
    fn put_object_tagging(
        &self,
        input: PutObjectTaggingRequest,
    ) -> RusotoFuture<PutObjectTaggingOutput, PutObjectTaggingError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TaggingSerializer::serialize(&mut writer, "Tagging", &input.tagging);
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutObjectTaggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(PutObjectTaggingOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Restores an archived copy of an object back into Amazon S3</p>
    #[allow(unused_variables, warnings)]
    fn restore_object(
        &self,
        input: RestoreObjectRequest,
    ) -> RusotoFuture<RestoreObjectOutput, RestoreObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("restore");
        request.set_params(params);
        if input.restore_request.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            RestoreRequestSerializer::serialize(
                &mut writer,
                "RestoreRequest",
                input.restore_request.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RestoreObjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = RestoreObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(RestoreObjectOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(restore_output_path) = response.headers.get("x-amz-restore-output-path")
                {
                    let value = restore_output_path.to_owned();
                    result.restore_output_path = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>This operation filters the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must also specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records, and returns only records that match the specified SQL expression. You must also specify the data serialization format for the response.</p>
    #[allow(unused_variables, warnings)]
    fn select_object_content(
        &self,
        input: SelectObjectContentRequest,
    ) -> RusotoFuture<SelectObjectContentOutput, SelectObjectContentError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put("select&select-type", "2");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        SelectObjectContentRequestSerializer::serialize(
            &mut writer,
            "SelectObjectContentRequest",
            &input,
            "http://s3.amazonaws.com/doc/2006-03-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SelectObjectContentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = SelectObjectContentOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(SelectObjectContentOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        })
    }

    /// <p>Uploads a part in a multipart upload.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    #[allow(unused_variables, warnings)]
    fn upload_part(
        &self,
        input: UploadPartRequest,
    ) -> RusotoFuture<UploadPartOutput, UploadPartError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_length) = input.content_length {
            request.add_header("Content-Length", &content_length.to_string());
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put("partNumber", &input.part_number);
        params.put("uploadId", &input.upload_id);
        request.set_params(params);
        if let Some(__body) = input.body {
            request.set_payload_stream(__body.len, __body.inner);
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UploadPartError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UploadPartOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UploadPartOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                Ok(result)
            }))
        })
    }

    /// <p>Uploads a part by copying data from an existing object as data source.</p>
    #[allow(unused_variables, warnings)]
    fn upload_part_copy(
        &self,
        input: UploadPartCopyRequest,
    ) -> RusotoFuture<UploadPartCopyOutput, UploadPartCopyError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        request.add_header("x-amz-copy-source", &input.copy_source);

        if let Some(ref copy_source_if_match) = input.copy_source_if_match {
            request.add_header(
                "x-amz-copy-source-if-match",
                &copy_source_if_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_modified_since) = input.copy_source_if_modified_since {
            request.add_header(
                "x-amz-copy-source-if-modified-since",
                &copy_source_if_modified_since.to_string(),
            );
        }

        if let Some(ref copy_source_if_none_match) = input.copy_source_if_none_match {
            request.add_header(
                "x-amz-copy-source-if-none-match",
                &copy_source_if_none_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
            request.add_header(
                "x-amz-copy-source-if-unmodified-since",
                &copy_source_if_unmodified_since.to_string(),
            );
        }

        if let Some(ref copy_source_range) = input.copy_source_range {
            request.add_header("x-amz-copy-source-range", &copy_source_range.to_string());
        }

        if let Some(ref copy_source_sse_customer_algorithm) =
            input.copy_source_sse_customer_algorithm
        {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-algorithm",
                &copy_source_sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key",
                &copy_source_sse_customer_key.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key-MD5",
                &copy_source_sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put("partNumber", &input.part_number);
        params.put("uploadId", &input.upload_id);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UploadPartCopyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UploadPartCopyOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UploadPartCopyOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(copy_source_version_id) =
                    response.headers.get("x-amz-copy-source-version-id")
                {
                    let value = copy_source_version_id.to_owned();
                    result.copy_source_version_id = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                Ok(result)
            }))
        })
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_s3_create_bucket() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "s3-create-bucket.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateBucketRequest::default();
        let result = client.create_bucket(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_error_s3_list_objects() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "s3-list-objects.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectsRequest::default();
        let result = client.list_objects(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_acl() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-get-bucket-acl.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketAclRequest::default();
        let result = client.get_bucket_acl(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_location() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-get-bucket-location.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketLocationRequest::default();
        let result = client.get_bucket_location(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_logging() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-get-bucket-logging.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketLoggingRequest::default();
        let result = client.get_bucket_logging(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_policy() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-get-bucket-policy.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketPolicyRequest::default();
        let result = client.get_bucket_policy(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_list_buckets() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-list-buckets.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.list_buckets().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_list_multipart_uploads() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-list-multipart-uploads.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListMultipartUploadsRequest::default();
        let result = client.list_multipart_uploads(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_list_object_versions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-list-object-versions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectVersionsRequest::default();
        let result = client.list_object_versions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_list_objects() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-list-objects.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectsRequest::default();
        let result = client.list_objects(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
