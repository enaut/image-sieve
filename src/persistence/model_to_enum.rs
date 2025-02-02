use num_traits::{FromPrimitive, ToPrimitive};
use slint::{Model, ModelRc, SharedString};

/// Convert a value from a slint SharedString model to an enum by mapping the model index to the enum value.
pub fn model_to_enum<Enum>(model: &ModelRc<SharedString>, value: &SharedString) -> Enum
where
    Enum: FromPrimitive,
{
    let mut enum_value: Enum = FromPrimitive::from_usize(0).unwrap();
    for index in 0..model.row_count() {
        if model.row_data(index).unwrap() == value {
            enum_value = FromPrimitive::from_usize(index).unwrap()
        }
    }
    enum_value
}

/// Convert an enum to a value from a slint SharedString modelby mapping the enum value to the model index.
pub fn enum_to_model<Enum>(model: &ModelRc<SharedString>, value: &Enum) -> SharedString
where
    Enum: ToPrimitive,
{
    let enum_value: u32 = ToPrimitive::to_u32(value).unwrap();
    model.row_data(enum_value as usize).unwrap()
}
