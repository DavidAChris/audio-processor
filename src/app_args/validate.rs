use std::path::Path;

const SECTION_VALUES: [i32; 8] = [1, 2, 4, 6, 10, 30, 80, 100];

pub(super) fn check_accuracy_value(value: &str) -> Result<i32, String> {
    let accuracy = value
        .parse()
        .map_err(|_| format!("`{}` is not a supported accuracy value", value))?;
    if SECTION_VALUES.contains(&accuracy) {
        Ok(accuracy)
    } else {
        Err(format!(
            "Please select one of the available values {:?}",
            SECTION_VALUES
        ))
    }
}

pub(super) fn check_wav_file(value: &str) -> Result<String, String> {
    let file_len = value.len();
    let wav_file = &value[file_len - 4..];
    let does_exist = Path::new(value).exists();
    if wav_file.contains(".wav") && file_len > 4 {
        if does_exist {
            Ok(String::from(value))
        } else {
            Err(format!("\nCannot find file '{}'", value))
        }
    } else {
        Err(String::from("\nPlease provider a valid .wav file"))
    }
}
