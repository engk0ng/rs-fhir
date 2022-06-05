#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Coding::Coding;
use crate::models::r5::Duration::Duration;
use crate::models::r5::Element::Element;
use crate::models::r5::Encounter_ClassHistory::Encounter_ClassHistory;
use crate::models::r5::Encounter_Diagnosis::Encounter_Diagnosis;
use crate::models::r5::Encounter_Hospitalization::Encounter_Hospitalization;
use crate::models::r5::Encounter_Location::Encounter_Location;
use crate::models::r5::Encounter_Participant::Encounter_Participant;
use crate::models::r5::Encounter_StatusHistory::Encounter_StatusHistory;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Identifier::Identifier;
use crate::models::r5::Meta::Meta;
use crate::models::r5::Narrative::Narrative;
use crate::models::r5::Period::Period;
use crate::models::r5::Reference::Reference;
use crate::models::r5::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Encounter<'_> {
    pub fn new(value: &Value) -> Encounter {
        Encounter {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for plannedEndDate
    pub fn _planned_end_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_plannedEndDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for plannedStartDate
    pub fn _planned_start_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_plannedStartDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The set of accounts that may be used for billing for this Encounter.
    pub fn account(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("account") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The actual start and end time of the encounter.
    pub fn actual_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("actualPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The appointment that scheduled this encounter.
    pub fn appointment(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("appointment") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The request this encounter satisfies (e.g. incoming referral or procedure
    /// request).
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Concepts representing classification of patient encounter such as ambulatory
    /// (outpatient), inpatient, emergency, home health or others due to local variations.
    pub fn class(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["class"]),
        }
    }

    /// The class history permits the tracking of the encounters transitions without
    /// needing to go  through the resource history.  This would be used for a case
    /// where an admission starts of as an emergency encounter, then transitions into an
    /// inpatient scenario. Doing this and not restarting a new encounter ensures that
    /// any lab/diagnostic results can more easily follow the patient and not require re-
    /// processing and not get lost or cancelled during a kind of discharge from emergency
    /// to inpatient.
    pub fn class_history(&self) -> Option<Vec<Encounter_ClassHistory>> {
        if let Some(Value::Array(val)) = self.value.get("classHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_ClassHistory {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource that
    /// contains them - they cannot be identified independently, nor can they have their
    /// own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The list of diagnosis relevant to this encounter.
    pub fn diagnosis(&self) -> Option<Vec<Encounter_Diagnosis>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosis") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_Diagnosis {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where a specific encounter should be classified as a part of a specific episode(s)
    /// of care this field should be used. This association can facilitate grouping of
    /// related encounters together for a specific purpose, such as government reporting,
    /// issue tracking, association via a common problem.  The association is recorded on
    /// the encounter as these are typically created after the episode of care and grouped
    /// on entry rather than editing the episode of care to append another encounter to it
    /// (the episode of care could span years).
    pub fn episode_of_care(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("episodeOfCare") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// Details about the admission to a healthcare service.
    pub fn hospitalization(&self) -> Option<Encounter_Hospitalization> {
        if let Some(val) = self.value.get("hospitalization") {
            return Some(Encounter_Hospitalization {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Identifier(s) by which this encounter is known.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often, this
    /// is a reference to an implementation guide that defines the special rules along
    /// with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Quantity of time the encounter lasted. This excludes the time during leaves of
    /// absence.
    pub fn length(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("length") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// List of locations where  the patient has been during this encounter.
    pub fn location(&self) -> Option<Vec<Encounter_Location>> {
        if let Some(Value::Array(val)) = self.value.get("location") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_Location {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with version
    /// changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's descendants.
    /// Usually modifier elements provide negation or qualification. To make the use of
    /// extensions safe and manageable, there is a strict set of governance applied to
    /// the definition and use of extensions. Though any implementer is allowed to define
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

    /// Another Encounter of which this encounter is a part of (administratively or in
    /// time).
    pub fn part_of(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("partOf") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The list of people responsible for providing the service.
    pub fn participant(&self) -> Option<Vec<Encounter_Participant>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_Participant {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The planned end date/time (or discharge date) of the encounter.
    pub fn planned_end_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("plannedEndDate") {
            return Some(string);
        }
        return None;
    }

    /// The planned start date/time (or admission date) of the encounter.
    pub fn planned_start_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("plannedStartDate") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the urgency of the encounter.
    pub fn priority(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("priority") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Reason the encounter takes place, expressed as a code or a reference to another
    /// resource. For admissions, this can be used for a coded admission diagnosis.
    pub fn reason(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The organization that is primarily responsible for this Encounter's services.
    /// This MAY be the same as the organization on the Patient record, however it could
    /// be different, such as if the actor performing the services was from an external
    /// organization (which may be billed seperately) for an external consultation.  Refer
    /// to the example bundle showing an abbreviated set of Encounters for a colonoscopy.
    pub fn service_provider(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("serviceProvider") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Broad categorization of the service that is to be provided (e.g. cardiology).
    pub fn service_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("serviceType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// planned | in-progress | onhold | completed | cancelled | entered-in-error |
    /// unknown.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// The status history permits the encounter resource to contain the status history
    /// without needing to read through the historical versions of the resource, or even
    /// have the server store them.
    pub fn status_history(&self) -> Option<Vec<Encounter_StatusHistory>> {
        if let Some(Value::Array(val)) = self.value.get("statusHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_StatusHistory {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The patient or group present at the encounter.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The subjectStatus value can be used to track the patient's status within the
    /// encounter. It details whether the patient has arrived or departed, has been
    /// triaged or is currently in a waiting status.
    pub fn subject_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectStatus") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be used
    /// to represent the content of the resource to a human. The narrative need not encode
    /// all the structured data, but is required to contain sufficient detail to make it
    /// "clinically safe" for a human to just read the narrative. Resource definitions
    /// may define what content should be represented in the narrative to ensure clinical
    /// safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, skilled
    /// nursing, rehabilitation).
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._planned_end_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._planned_start_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.account() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.actual_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.appointment() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.based_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.class().validate() {
            return false;
        }
        if let Some(_val) = self.class_history() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.diagnosis() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.episode_of_care() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.hospitalization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.length() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.location() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.part_of() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.participant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.planned_end_date() {}
        if let Some(_val) = self.planned_start_date() {}
        if let Some(_val) = self.priority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.service_provider() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.service_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_history() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.subject() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subject_status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct EncounterBuilder {
    pub(crate) value: Value,
}

impl EncounterBuilder {
    pub fn build(&self) -> Encounter {
        Encounter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Encounter) -> EncounterBuilder {
        EncounterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(class: Coding) -> EncounterBuilder {
        let mut __value: Value = json!({});
        __value["class"] = json!(class.value);
        return EncounterBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut EncounterBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut EncounterBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _planned_end_date<'a>(&'a mut self, val: Element) -> &'a mut EncounterBuilder {
        self.value["_plannedEndDate"] = json!(val.value);
        return self;
    }

    pub fn _planned_start_date<'a>(&'a mut self, val: Element) -> &'a mut EncounterBuilder {
        self.value["_plannedStartDate"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut EncounterBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn account<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut EncounterBuilder {
        self.value["account"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn actual_period<'a>(&'a mut self, val: Period) -> &'a mut EncounterBuilder {
        self.value["actualPeriod"] = json!(val.value);
        return self;
    }

    pub fn appointment<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut EncounterBuilder {
        self.value["appointment"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn based_on<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut EncounterBuilder {
        self.value["basedOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn class_history<'a>(
        &'a mut self,
        val: Vec<Encounter_ClassHistory>,
    ) -> &'a mut EncounterBuilder {
        self.value["classHistory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut EncounterBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn diagnosis<'a>(&'a mut self, val: Vec<Encounter_Diagnosis>) -> &'a mut EncounterBuilder {
        self.value["diagnosis"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn episode_of_care<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut EncounterBuilder {
        self.value["episodeOfCare"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut EncounterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn hospitalization<'a>(
        &'a mut self,
        val: Encounter_Hospitalization,
    ) -> &'a mut EncounterBuilder {
        self.value["hospitalization"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EncounterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut EncounterBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut EncounterBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut EncounterBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn length<'a>(&'a mut self, val: Duration) -> &'a mut EncounterBuilder {
        self.value["length"] = json!(val.value);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Vec<Encounter_Location>) -> &'a mut EncounterBuilder {
        self.value["location"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut EncounterBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut EncounterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Reference) -> &'a mut EncounterBuilder {
        self.value["partOf"] = json!(val.value);
        return self;
    }

    pub fn participant<'a>(
        &'a mut self,
        val: Vec<Encounter_Participant>,
    ) -> &'a mut EncounterBuilder {
        self.value["participant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn planned_end_date<'a>(&'a mut self, val: &str) -> &'a mut EncounterBuilder {
        self.value["plannedEndDate"] = json!(val);
        return self;
    }

    pub fn planned_start_date<'a>(&'a mut self, val: &str) -> &'a mut EncounterBuilder {
        self.value["plannedStartDate"] = json!(val);
        return self;
    }

    pub fn priority<'a>(&'a mut self, val: CodeableConcept) -> &'a mut EncounterBuilder {
        self.value["priority"] = json!(val.value);
        return self;
    }

    pub fn reason<'a>(&'a mut self, val: Vec<CodeableReference>) -> &'a mut EncounterBuilder {
        self.value["reason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn service_provider<'a>(&'a mut self, val: Reference) -> &'a mut EncounterBuilder {
        self.value["serviceProvider"] = json!(val.value);
        return self;
    }

    pub fn service_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut EncounterBuilder {
        self.value["serviceType"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut EncounterBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn status_history<'a>(
        &'a mut self,
        val: Vec<Encounter_StatusHistory>,
    ) -> &'a mut EncounterBuilder {
        self.value["statusHistory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Reference) -> &'a mut EncounterBuilder {
        self.value["subject"] = json!(val.value);
        return self;
    }

    pub fn subject_status<'a>(&'a mut self, val: CodeableConcept) -> &'a mut EncounterBuilder {
        self.value["subjectStatus"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut EncounterBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut EncounterBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
