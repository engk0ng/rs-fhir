#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Period::Period;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::Range::Range;
use crate::models::r5::Ratio::Ratio;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of an event relevant for purposes such as operations, privacy, security,
/// maintenance, and performance analysis.

#[derive(Debug)]
pub struct AuditEvent_Detail<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AuditEvent_Detail<'_> {
    pub fn new(value: &Value) -> AuditEvent_Detail {
        AuditEvent_Detail {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for valueBase64Binary
    pub fn _value_base_64_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBase64Binary") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueDateTime
    pub fn _value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueInteger
    pub fn _value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInteger") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueTime
    pub fn _value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueTime") {
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

    /// The type of extra detail provided in the value.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// The  value of the extra detail.
    pub fn value_base_64_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueBase64Binary") {
            return Some(string);
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("valuePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("valueRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// The  value of the extra detail.
    pub fn value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueTime") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_base_64_binary() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_integer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_time() {
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
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.value_base_64_binary() {}
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.value_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_ratio() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self.value_time() {}
        return true;
    }
}

#[derive(Debug)]
pub struct AuditEvent_DetailBuilder {
    pub(crate) value: Value,
}

impl AuditEvent_DetailBuilder {
    pub fn build(&self) -> AuditEvent_Detail {
        AuditEvent_Detail {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AuditEvent_Detail) -> AuditEvent_DetailBuilder {
        AuditEvent_DetailBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> AuditEvent_DetailBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return AuditEvent_DetailBuilder { value: __value };
    }

    pub fn _value_base_64_binary<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut AuditEvent_DetailBuilder {
        self.value["_valueBase64Binary"] = json!(val.value);
        return self;
    }

    pub fn _value_boolean<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_DetailBuilder {
        self.value["_valueBoolean"] = json!(val.value);
        return self;
    }

    pub fn _value_date_time<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_DetailBuilder {
        self.value["_valueDateTime"] = json!(val.value);
        return self;
    }

    pub fn _value_integer<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_DetailBuilder {
        self.value["_valueInteger"] = json!(val.value);
        return self;
    }

    pub fn _value_string<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_DetailBuilder {
        self.value["_valueString"] = json!(val.value);
        return self;
    }

    pub fn _value_time<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_DetailBuilder {
        self.value["_valueTime"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AuditEvent_DetailBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_DetailBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AuditEvent_DetailBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_base_64_binary<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueBase64Binary"] = json!(val);
        return self;
    }

    pub fn value_boolean<'a>(&'a mut self, val: bool) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueBoolean"] = json!(val);
        return self;
    }

    pub fn value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn value_date_time<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueDateTime"] = json!(val);
        return self;
    }

    pub fn value_integer<'a>(&'a mut self, val: f64) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueInteger"] = json!(val);
        return self;
    }

    pub fn value_period<'a>(&'a mut self, val: Period) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valuePeriod"] = json!(val.value);
        return self;
    }

    pub fn value_quantity<'a>(&'a mut self, val: Quantity) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }

    pub fn value_range<'a>(&'a mut self, val: Range) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueRange"] = json!(val.value);
        return self;
    }

    pub fn value_ratio<'a>(&'a mut self, val: Ratio) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueRatio"] = json!(val.value);
        return self;
    }

    pub fn value_string<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueString"] = json!(val);
        return self;
    }

    pub fn value_time<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_DetailBuilder {
        self.value["valueTime"] = json!(val);
        return self;
    }
}
