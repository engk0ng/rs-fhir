#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or modify
/// physiological functions by exerting a pharmacological, immunological or metabolic
/// action. This resource is intended to define and detail such products and their
/// properties, for uses other than direct patient care (e.g. regulatory use, or drug
/// catalogs).

#[derive(Debug)]
pub struct MedicinalProductDefinition_NamePart<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductDefinition_NamePart<'_> {
    pub fn new(value: &Value) -> MedicinalProductDefinition_NamePart {
        MedicinalProductDefinition_NamePart {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for part
    pub fn _part(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_part") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may be
    /// any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element
    /// in which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To make
    /// the use of extensions safe and manageable, there is a strict set of governance
    /// applied to the definition and use of extensions. Though any implementer can define
    /// an extension, there is a set of requirements that SHALL be met as part of the
    /// definition of the extension. Applications processing a resource are required to
    /// check for modifier extensions.    Modifier extensions SHALL NOT change the meaning
    /// of any elements on Resource or DomainResource (including cannot change the meaning
    /// of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A fragment of a product name.
    pub fn part(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("part") {
            return Some(string);
        }
        return None;
    }

    /// Identifying type for this part of the name (e.g. strength part).
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._part() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.part() {}
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductDefinition_NamePartBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductDefinition_NamePartBuilder {
    pub fn build(&self) -> MedicinalProductDefinition_NamePart {
        MedicinalProductDefinition_NamePart {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductDefinition_NamePart,
    ) -> MedicinalProductDefinition_NamePartBuilder {
        MedicinalProductDefinition_NamePartBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> MedicinalProductDefinition_NamePartBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return MedicinalProductDefinition_NamePartBuilder { value: __value };
    }

    pub fn _part<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductDefinition_NamePartBuilder {
        self.value["_part"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinition_NamePartBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinition_NamePartBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductDefinition_NamePartBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn part<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductDefinition_NamePartBuilder {
        self.value["part"] = json!(val);
        return self;
    }
}
