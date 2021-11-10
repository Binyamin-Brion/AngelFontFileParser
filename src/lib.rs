use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Helps to set all of the variables associated with a CharacterInformation struct
macro_rules! set_char_values
{
    ($char_info: expr, $line: expr, $($member: tt, $member_expr: expr),*) =>
    {
        if let Some((identifier, value)) = extract_numeric_value($line)
        {
            // When comparing the member value, the input to the macro must be an expression.
            // However, using an expression when assigning a member variable does not work.
            // Hence the member / member value is passed in as a tt and a expression

            $(
                if identifier == $member_expr
                {
                    $char_info.$member = Some(value);
                }
            )+
        }
    };
}

/// Extracts the required information to query the associated texture atlas [of the passed in font file]
/// as well as render those characters onto a screen
///
/// `file_location` - the location of the file in the angel file format
fn extract_characters<A: AsRef<Path> + Debug + Clone>(file_location: A) -> Result<Vec<CharacterInfo>, String>
{
    // Attempting to open the file specified by file location consumes the file location variable.
    // This is an issue when creating the error message if file opening failed
    let file_location_copy = file_location.clone();

    let file = match File::open(file_location)
    {
        Ok(i) => i,
        Err(err) =>
            {
                // The default error message, err, is not that great- does not provide the location of the file that could not be opened
                return Err(format!("Unable to open file {:?}, with the error: {}", file_location_copy, err));
            }
    };

    let reader = BufReader::new(file);

    let mut characters = Vec::new();

    for (index, read_line) in reader.lines().enumerate()
    {
        let line = match read_line
        {
            Ok(i) => i,
            Err(err) =>
                {
                    // Custom error message to have more information than the default err information
                    return Err(format!("Unable to read line number {} with error: {}", index, err));
                }
        };

        // Only interested in file containing character information, not background information such
        // as number of characters, name of the font, etc
        if !line.starts_with("char id")
        {
            continue;
        }

        let mut char_info = CharacterInfo::new();

        for split_result in line.split_whitespace().filter(|x| x.contains('='))
        {
            // All required information has the form of [memberName]=[value]

            set_char_values!(char_info, split_result,
                id, "id",
                x, "x",
                y, "y",
                width, "width",
                height, "height",
                x_offset, "xoffset",
                y_offset, "yoffset",
                x_advance, "xadvance",
                page, "page",
                chnl, "chnl");
        }

        characters.push(char_info);
    }

    Ok(characters)
}

/// Extracts the given string into two outputs: the name of the variable related to the font and the
/// value of that variable
///
/// `input` - the memberVariable-value string extracted from the font file
fn extract_numeric_value(input: &str) -> Option<(String, i32)>
{
    let mut result = ("".to_string(), 0);

    // Should only be two possible split results if input is of the form of [variable]=[value]
    if input.split('=').count() != 2
    {
        return None;
    }

    for (index, x) in input.split('=').enumerate()
    {
        if index == 0
        {
            result.0 = x.to_string();
        }

        if index == 1
        {
            match x.parse::<i32>()
            {
                Ok(i) => result.1 = i,
                Err(_) => return None
            }
        }
    }

    Some(result)
}

/// Stores the information required to extract a character from the associated texture atlas [of the
/// passed in font file] as well as render the character to a screen
#[derive(Debug)]
pub struct CharacterInfo
{
    pub id: Option<i32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub x_offset: Option<i32>,
    pub y_offset: Option<i32>,
    pub x_advance: Option<i32>,
    pub page: Option<i32>,
    pub chnl: Option<i32>
}

impl CharacterInfo
{
    /// Creates a default character with no usable information
    fn new() -> CharacterInfo
    {
        CharacterInfo
        {
            id: None,
            x: None,
            y: None,
            width: None,
            height: None,
            x_offset: None,
            y_offset: None,
            x_advance: None,
            page: None,
            chnl: None
        }
    }
}


#[cfg(test)]
mod tests
{
    use std::env;
    use std::path::PathBuf;
    use crate::{CharacterInfo, extract_characters};

    /*
        All tests contain three characters. Each test that involves an modified input such that it
        does not follow the expected format (might not always be invalid though- for example, additional
        character parameters are ignored) modify the second listed character. This is to ensure that
        unexpectedly formatted lines do not affect lines above or below
     */

    fn get_test_folder() -> PathBuf
    {
        let path = env::current_dir().unwrap();
        path.join("test_files")
    }

    fn validate_untouched_char(char_info: &CharacterInfo)
    {
        assert_eq!(Some(124), char_info.id);
        assert_eq!(Some(0), char_info.x);
        assert_eq!(Some(0), char_info.y);
        assert_eq!(Some(22), char_info.width);
        assert_eq!(Some(72), char_info.height);
        assert_eq!(Some(-3), char_info.x_offset);
        assert_eq!(Some(3), char_info.y_offset);
        assert_eq!(Some(30), char_info.x_advance);
        assert_eq!(Some(0), char_info.page);
        assert_eq!(Some(0), char_info.chnl);
    }

    fn check_all_valid_characters(characters: Vec<CharacterInfo>)
    {
        assert_eq!(3, characters.len());

        let second_char = &characters[1];
        assert_eq!(Some(32), second_char.id);
        assert_eq!(Some(0), second_char.x);
        assert_eq!(Some(0), second_char.y);
        assert_eq!(Some(0), second_char.width);
        assert_eq!(Some(0), second_char.height);
        assert_eq!(Some(0), second_char.x_offset);
        assert_eq!(Some(53), second_char.y_offset);
        assert_eq!(Some(32), second_char.x_advance);
        assert_eq!(Some(0), second_char.page);
        assert_eq!(Some(0), second_char.chnl);

        validate_untouched_char(&characters[0]);
        validate_untouched_char(&characters[2]);
    }

    #[test]
    fn check_valid_file()
    {
        let test_file = get_test_folder().join("validFormat.fnt");
        let characters = extract_characters(test_file).unwrap();
        check_all_valid_characters(characters);
    }

    #[test]
    fn check_added_parameter()
    {
        let test_file = get_test_folder().join("added_parameter.fnt");
        let characters = extract_characters(test_file).unwrap();
        check_all_valid_characters(characters);
    }

    #[test]
    fn check_missing_parameters()
    {
        let test_file = get_test_folder().join("missing_parameters.fnt");
        let characters = extract_characters(test_file).unwrap();

        assert_eq!(3, characters.len());

        let second_char = &characters[1];
        assert_eq!(Some(32), second_char.id);
        assert_eq!(None, second_char.x);
        assert_eq!(None, second_char.y);
        assert_eq!(Some(0), second_char.width);
        assert_eq!(Some(0), second_char.height);
        assert_eq!(Some(0), second_char.x_offset);
        assert_eq!(Some(53), second_char.y_offset);
        assert_eq!(Some(32), second_char.x_advance);
        assert_eq!(Some(0), second_char.page);
        assert_eq!(Some(0), second_char.chnl);

        validate_untouched_char(&characters[0]);
        validate_untouched_char(&characters[2]);
    }

    #[test]
    fn check_mispelled_parameters()
    {
        let test_file = get_test_folder().join("mispelled_parameters.fnt");
        let characters = extract_characters(test_file).unwrap();

        assert_eq!(3, characters.len());

        let second_char = &characters[1];
        assert_eq!(Some(32), second_char.id);
        assert_eq!(Some(0), second_char.x);
        assert_eq!(None, second_char.y);
        assert_eq!(None, second_char.width);
        assert_eq!(None, second_char.height);
        assert_eq!(None, second_char.x_offset);
        assert_eq!(None, second_char.y_offset);
        assert_eq!(None, second_char.x_advance);
        assert_eq!(None, second_char.page);
        assert_eq!(None, second_char.chnl);

        validate_untouched_char(&characters[0]);
        validate_untouched_char(&characters[2]);
    }

    #[test]
    fn check_unrecognized_line()
    {
        let test_file = get_test_folder().join("unrecognized_line.fnt");
        let characters = extract_characters(test_file).unwrap();

        // Line is unrecognized since character line does not start with char id

        assert_eq!(2, characters.len());
        validate_untouched_char(&characters[0]);
        validate_untouched_char(&characters[1]);
    }

    #[test]
    fn check_incorrect_format()
    {
        let test_file = get_test_folder().join("incorrect_format.fnt");

        let characters = extract_characters(test_file).unwrap();

        assert_eq!(3, characters.len());

        let second_char = &characters[1];
        assert_eq!(Some(32), second_char.id);
        assert_eq!(Some(0), second_char.x);
        assert_eq!(Some(0), second_char.y);
        assert_eq!(Some(0), second_char.width);
        assert_eq!(Some(0), second_char.height);
        assert_eq!(Some(0), second_char.x_offset);
        assert_eq!(None, second_char.y_offset);
        assert_eq!(None, second_char.x_advance);
        assert_eq!(Some(0), second_char.page);
        assert_eq!(Some(0), second_char.chnl);

        validate_untouched_char(&characters[0]);
        validate_untouched_char(&characters[2]);
    }
}