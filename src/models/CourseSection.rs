use serde::{Deserialize, Serialize};

#[path = "Instructor.rs"]
mod Instructor;
#[path = "TimeLocation.rs"]
mod TimeLocation;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CourseSection {
    pub class_closed: Option<String>,
    // pub concurrent_courses: Option<Vec<Course>>,
    pub course_cancelled: Option<String>,
    pub department_approval_required: Option<bool>,
    pub enroll_code: Option<String>,
    pub enrolled_total: Option<u32>,
    pub grading_option_code: Option<String>,
    pub instructor_approval_required: bool,
    pub instructors: Vec<Instructor::Instructor>,
    pub max_enroll: Option<u32>,
    pub restriction_level: Option<String>,
    pub restriction_major: Option<String>,
    pub restriction_major_pass: Option<String>,
    pub restriction_minor: Option<String>,
    pub restriction_minor_pass: Option<String>,
    pub secondary_status: Option<String>,
    pub section: Option<String>,
    pub session: Option<String>,
    pub time_locations: Vec<TimeLocation::TimeLocation>,
}
