#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A process where a researcher or organization plans and then executes a series
/// of steps intended to increase the field of healthcare-related knowledge.  This
/// includes studies of safety, efficacy, comparative effectiveness and other
/// information about medications, devices, therapies and other interventional and
/// investigative techniques. A ResearchStudy involves the gathering of information
/// about human or animal subjects or stability data about drug products or drug
/// substances.

#[derive(Debug)]
pub struct ResearchStudy_StatusDate<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ResearchStudy_StatusDate<'_> {
    pub fn new(value: &Value) -> ResearchStudy_StatusDate {
        ResearchStudy_StatusDate {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for actual
    pub fn _actual(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actual") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Label for status or state.
    pub fn activity(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["activity"]),
        }
    }

    /// Actual if true else anticipated.
    pub fn actual(&self) -> Option<bool> {
        if let Some(val) = self.value.get("actual") {
            return Some(val.as_bool().unwrap());
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

    /// Date range.
    pub fn period(&self) -> Period {
        Period {
            value: Cow::Borrowed(&self.value["period"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._actual() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.activity().validate() {
            return false;
        }
        if let Some(_val) = self.actual() {}
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
        if !self.period().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ResearchStudy_StatusDateBuilder {
    pub(crate) value: Value,
}

impl ResearchStudy_StatusDateBuilder {
    pub fn build(&self) -> ResearchStudy_StatusDate {
        ResearchStudy_StatusDate {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ResearchStudy_StatusDate) -> ResearchStudy_StatusDateBuilder {
        ResearchStudy_StatusDateBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(activity: CodeableConcept, period: Period) -> ResearchStudy_StatusDateBuilder {
        let mut __value: Value = json!({});
        __value["activity"] = json!(activity.value);
        __value["period"] = json!(period.value);
        return ResearchStudy_StatusDateBuilder { value: __value };
    }

    pub fn _actual<'a>(&'a mut self, val: Element) -> &'a mut ResearchStudy_StatusDateBuilder {
        self.value["_actual"] = json!(val.value);
        return self;
    }

    pub fn actual<'a>(&'a mut self, val: bool) -> &'a mut ResearchStudy_StatusDateBuilder {
        self.value["actual"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchStudy_StatusDateBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ResearchStudy_StatusDateBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchStudy_StatusDateBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
