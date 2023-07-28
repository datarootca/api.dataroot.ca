use validator::ValidationError;

use crate::{api::{config, resources::event::dto::RequestFindEvent}, repository::event::{EventStatusOption, DateRangeOption}};


pub fn validate_event_status_option(value: &EventStatusOption) -> Result<(), ValidationError> {
    match value {
        EventStatusOption::Upcomming => Ok(()),
        EventStatusOption::Recurrent => Ok(()),
        EventStatusOption::Past => Ok(()),
    }
}

pub fn validate_event_request(request: &RequestFindEvent) -> Result<(), ValidationError> {
    match &request.time_frame {
        Some(DateRangeOption::Custom) => {
            match (request.start_date, request.end_date) {
                (Some(start_date), Some(end_date)) => {
                    if start_date > end_date {
                        return Err(ValidationError::new("end date should be later than start date"));
                    }
                },
                _ => return Err(ValidationError::new("start and end dates must be provided for custom time frame"))
            }
        },
        Some(DateRangeOption::Today) | Some(DateRangeOption::ThisWeek) | Some(DateRangeOption::ThisMonth) => {
            if request.start_date.is_some() || request.end_date.is_some() {
                return Err(ValidationError::new("start and end dates must be None for non-custom time frames"));
            }
        },
        None => {
            // time_frame is None, no validation necessary
        },
    }
    Ok(())
}


pub fn validate_page_size_max(page_size: u32) -> Result<(), ValidationError> {
    if page_size > config::get_config().page_size_max {
        return Err(ValidationError::new("page_size greater than the max"));
    }
    Ok(())
}
