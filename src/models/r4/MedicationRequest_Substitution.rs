#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4::CodeableConcept::CodeableConcept;
use crate::models::r4::Element::Element;
use crate::models::r4::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to
/// generalize the use across inpatient and outpatient settings, including care plans,
/// etc., and to harmonize with workflow patterns.

#[derive(Debug)]
pub struct MedicationRequest_Substitution<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationRequest_Substitution<'_> {
    pub fn new(value: &Value) -> MedicationRequest_Substitution {
        MedicationRequest_Substitution {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for allowedBoolean
    pub fn _allowed_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allowedBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// True if the prescriber allows a different drug to be dispensed from what was
    /// prescribed.
    pub fn allowed_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("allowedBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// True if the prescriber allows a different drug to be dispensed from what was
    /// prescribed.
    pub fn allowed_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("allowedCodeableConcept") {
            return Some(CodeableConcept {
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

    /// Indicates the reason for the substitution, or why substitution must or must not
    /// be performed.
    pub fn reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._allowed_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.allowed_boolean() {}
        if let Some(_val) = self.allowed_codeable_concept() {
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
        if let Some(_val) = self.reason() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationRequest_SubstitutionBuilder {
    pub(crate) value: Value,
}

impl MedicationRequest_SubstitutionBuilder {
    pub fn build(&self) -> MedicationRequest_Substitution {
        MedicationRequest_Substitution {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationRequest_Substitution) -> MedicationRequest_SubstitutionBuilder {
        MedicationRequest_SubstitutionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationRequest_SubstitutionBuilder {
        let mut __value: Value = json!({});
        return MedicationRequest_SubstitutionBuilder { value: __value };
    }

    pub fn _allowed_boolean<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationRequest_SubstitutionBuilder {
        self.value["_allowedBoolean"] = json!(val.value);
        return self;
    }

    pub fn allowed_boolean<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut MedicationRequest_SubstitutionBuilder {
        self.value["allowedBoolean"] = json!(val);
        return self;
    }

    pub fn allowed_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationRequest_SubstitutionBuilder {
        self.value["allowedCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationRequest_SubstitutionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationRequest_SubstitutionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationRequest_SubstitutionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationRequest_SubstitutionBuilder {
        self.value["reason"] = json!(val.value);
        return self;
    }
}
